import browser from "webextension-polyfill";
import * as Sentry from "@sentry/react";

import {
  assertUrl,
  assertUrlPattern,
  EXTENSION_STEP,
  MessageToExtension,
  MessageToExtensionType,
  ExtensionInternalMessageType,
  isExtensionInternalMessage,
  ZkProvingStatus,
  MessageFromExtensionType,
  isMessageToExtension,
  isLegacyPingMessage,
} from "./web-proof-commons";

import { WebProverSessionContextManager } from "./state/webProverSessionContext";
import { match, P } from "ts-pattern";
import { zkProvingStatusStore } from "./state/zkProvingStatusStore.ts";
import { initSentry } from "./helpers/sentry.ts";
import { SIDE_PANEL_CONNECTION_NAME } from "constants/messaging.ts";

let port: browser.Runtime.Port | undefined = undefined;
let openedTabId: number | undefined = undefined;

initSentry();

// important sidePanel is chrome specific it doesn't exist in webExtension polyfill
void chrome.sidePanel.setPanelBehavior({ openPanelOnActionClick: true });

browser.runtime.onConnectExternal.addListener((connectedPort) => {
  port = connectedPort;
  port.onMessage.addListener((message: unknown) => {
    if (isLegacyPingMessage(message)) {
      port?.postMessage({
        type: MessageFromExtensionType.Pong,
        payload: {},
      });
      return;
    }
    if (!isMessageToExtension(message)) {
      return;
    }
    match(message)
      .with({ type: MessageToExtensionType.RequestWebProof }, (msg) => {
        void handleProofRequest(msg, connectedPort.sender);
      })
      .with({ type: MessageToExtensionType.NotifyZkProvingStatus }, (msg) => {
        void handleProvingStatusNotification(msg);
      })
      .with({ type: MessageToExtensionType.OpenSidePanel }, () => {
        void handleOpenSidePanel(connectedPort.sender);
      })
      .with({ type: MessageToExtensionType.CloseSidePanel }, () => {
        void handleCloseSidePanel();
      })
      .exhaustive();
  });
});

browser.runtime.onMessage.addListener(async (message: unknown) => {
  if (!isExtensionInternalMessage(message)) {
    return;
  }
  await match(message)
    .with(
      {
        type: P.union(
          ExtensionInternalMessageType.ProofDone,
          ExtensionInternalMessageType.ProofError,
          ExtensionInternalMessageType.StepCompleted,
        ),
      },
      () => {
        try {
          port?.postMessage(message);
        } catch (e) {
          console.error("Could not send message to webpage", e);
        }
      },
    )
    .with({ type: ExtensionInternalMessageType.RedirectBack }, async () => {
      if (openedTabId) {
        await browser.tabs.remove(openedTabId);
      }
      await browser.tabs.update(port?.sender?.tab?.id, { active: true });
      port?.postMessage(message);
    })
    .with({ type: ExtensionInternalMessageType.TabOpened }, ({ payload }) => {
      openedTabId = payload.tabId;
    })
    .with({ type: ExtensionInternalMessageType.ProofProcessing }, () => {
      port?.postMessage({
        type: ExtensionInternalMessageType.ProofProcessing,
        payload: {},
      });
    })

    //Two handler above are here to make sure we can safely do exhaustive match below
    //that shows that probably we should have one more messages cateegory which is internal but from background to the sidepanel

    .with(
      { type: ExtensionInternalMessageType.CleanProvingSessionStorageOnClose },
      () => {
        return new Promise((resolve) => {
          resolve(
            `${ExtensionInternalMessageType.CleanProvingSessionStorageOnClose} shouldnt be sent to background`,
          );
        });
      },
    )
    .with({ type: ExtensionInternalMessageType.CloseSidePanel }, () => {
      return new Promise((resolve) => {
        resolve(
          `${ExtensionInternalMessageType.CloseSidePanel} shouldnt be sent to background`,
        );
      });
    })
    .with({ type: ExtensionInternalMessageType.ResetTlsnProving }, () => {
      return new Promise((resolve) => {
        resolve(
          `${ExtensionInternalMessageType.ResetTlsnProving} shouldnt be sent to background`,
        );
      });
    })
    .exhaustive();
});

browser.runtime.onMessageExternal.addListener(
  (message: unknown, sender: browser.Runtime.MessageSender) => {
    if (isLegacyPingMessage(message)) {
      return new Promise((resolve) => {
        resolve({
          type: MessageFromExtensionType.Pong,
          payload: {},
        });
      });
    }
    if (!isMessageToExtension(message)) {
      return new Promise((_resolve, reject) => {
        reject(
          new Error(
            `Unknown message type: ${(message as { type: string }).type}`,
          ),
        );
      });
    }
    return match(message)
      .with({ type: MessageToExtensionType.RequestWebProof }, (msg) => {
        void handleProofRequest(msg, sender);
      })
      .with({ type: MessageToExtensionType.OpenSidePanel }, () => {
        void handleOpenSidePanel(sender);
      })
      .with({ type: MessageToExtensionType.CloseSidePanel }, () => {
        void handleCloseSidePanel();
      })
      .with({ type: MessageToExtensionType.NotifyZkProvingStatus }, (msg) => {
        void handleProvingStatusNotification(msg);
      })
      .otherwise(() => {
        return new Promise((_resolve, reject) => {
          reject(new Error(`${message.type} sent wrong channel`));
        });
      });
  },
);

const handleOpenSidePanel = async (sender?: browser.Runtime.MessageSender) => {
  // important sidePanel is chrome specific it doesn't exist in webExtension polyfill
  if (chrome.sidePanel && sender?.tab?.windowId) {
    await chrome.sidePanel.open({ windowId: sender.tab?.windowId });
  }
};

const handleCloseSidePanel = () => {
  void browser.runtime.sendMessage({
    type: ExtensionInternalMessageType.CloseSidePanel,
  });
};

const cleanProvingSessionStorageOnClose = () => {
  void browser.runtime.sendMessage({
    type: ExtensionInternalMessageType.CleanProvingSessionStorageOnClose,
  });
};

const handleProofRequest = async (
  message: Extract<
    MessageToExtension,
    { type: MessageToExtensionType.RequestWebProof }
  >,
  sender?: browser.Runtime.MessageSender,
) => {
  validateProofRequest(message);
  if (chrome.sidePanel && sender?.tab?.windowId) {
    await chrome.sidePanel.open({ windowId: sender.tab?.windowId });
  }
  await browser.storage.session.set({
    browsingHistory: [],
    zkProvingStatus: ZkProvingStatus.NotStarted,
  });

  await WebProverSessionContextManager.instance.setWebProverSessionConfig(
    message.payload,
  );

  void browser.runtime.sendMessage({
    type: ExtensionInternalMessageType.ResetTlsnProving,
  });

  if (Sentry.isInitialized()) {
    Sentry.setContext("WebProverSessionConfig", {
      notaryUrl: message.payload.notaryUrl,
      wsProxyUrl: message.payload.wsProxyUrl,
    });
  }
};

const handleProvingStatusNotification = async (
  message: Extract<
    MessageToExtension,
    { type: MessageToExtensionType.NotifyZkProvingStatus }
  >,
) => {
  await zkProvingStatusStore.setProvingStatus(message.payload);
  if (message.payload.status === ZkProvingStatus.Done) {
    cleanProvingSessionStorageOnClose();
  }
  if (Sentry.isInitialized()) {
    const severity: Sentry.SeverityLevel =
      message.payload.status === ZkProvingStatus.Error ? "error" : "info";
    Sentry.captureMessage(`Proof status: ${message.payload.status}`, severity);
  }
};

const validateProofRequest = (
  message: Extract<
    MessageToExtension,
    { type: MessageToExtensionType.RequestWebProof }
  >,
) => {
  try {
    message.payload.steps.forEach(({ step, url }) => {
      match({ step, url })
        .with({ step: EXTENSION_STEP.startPage }, ({ url }) => assertUrl(url))
        .with({ step: EXTENSION_STEP.redirect }, ({ url }) => assertUrl(url))
        .with({ step: EXTENSION_STEP.userAction }, ({ url }) => assertUrl(url))
        .with(
          {
            step: P.union(EXTENSION_STEP.notarize, EXTENSION_STEP.expectUrl),
          },
          ({ url }) => assertUrlPattern(url),
        )
        .with(
          {
            step: P.union(
              EXTENSION_STEP.extractVariables,
              EXTENSION_STEP.clickButton,
            ),
          },
          () => {
            console.warn("Unsupported step type: ", step);
          },
        )
        .exhaustive();
    });
  } catch (e) {
    console.error("Invalid message", e);
    throw e;
  }
};

// this monitors the sidepanel connection
// when the sidepanel is closed, it sends a message to the browser

browser.runtime.onConnect.addListener((sidePanelPort) => {
  if (sidePanelPort.name === SIDE_PANEL_CONNECTION_NAME) {
    sidePanelPort.onDisconnect.addListener(() => {
      port?.postMessage({
        type: MessageFromExtensionType.SidePanelClosed,
        payload: {},
      });
    });
  }
});
