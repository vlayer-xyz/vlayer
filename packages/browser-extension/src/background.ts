import browser from "webextension-polyfill";
import * as Sentry from "@sentry/react";

import {
  assertUrl,
  assertUrlPattern,
  EXTENSION_STEP,
  ExtensionAction,
  ExtensionMessage,
  ExtensionMessageType,
  MessageToExtension,
  ZkProvingStatus,
} from "./web-proof-commons";

import { WebProverSessionContextManager } from "./state/webProverSessionContext";
import { match, P } from "ts-pattern";
import { zkProvingStatusStore } from "./state/zkProvingStatusStore.ts";
import { initSentry } from "./helpers/sentry.ts";
import { SIDE_PANEL_CONNECTION_NAME } from "constants/messaging.ts";

let port: browser.Runtime.Port | undefined = undefined;
let openedTabId: number | undefined = undefined;

initSentry();

// @ts-expect-error https://github.com/wxt-dev/wxt/issues/570#issuecomment-2022365906
// eslint-disable-next-line
browser.sidePanel.setPanelBehavior({ openPanelOnActionClick: true });

browser.runtime.onConnectExternal.addListener((connectedPort) => {
  port = connectedPort;
  port.onMessage.addListener((message: MessageToExtension) => {
    match(message)
      .with({ action: ExtensionAction.RequestWebProof }, (msg) => {
        void handleProofRequest(msg, connectedPort.sender);
      })
      .with({ action: ExtensionAction.NotifyZkProvingStatus }, (msg) => {
        void handleProvingStatusNotification(msg);
      })
      .with({ action: ExtensionAction.OpenSidePanel }, () => {
        void handleOpenSidePanel(connectedPort.sender);
      })
      .with({ action: ExtensionAction.CloseSidePanel }, () => {
        void handleCloseSidePanel();
      })
      .exhaustive();
  });
});

browser.runtime.onMessage.addListener(async (message: ExtensionMessage) => {
  await match(message)
    .with(
      {
        type: P.union(
          ExtensionMessageType.ProofDone,
          ExtensionMessageType.ProofError,
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
    .with({ type: ExtensionMessageType.RedirectBack }, async () => {
      if (openedTabId) {
        await browser.tabs.remove(openedTabId);
      }
      await browser.tabs.update(port?.sender?.tab?.id, { active: true });
    })
    .with({ type: ExtensionMessageType.TabOpened }, ({ payload }) => {
      openedTabId = payload.tabId;
    })
    .with({ type: ExtensionMessageType.ProofProcessing }, () => {
      port?.postMessage({
        type: ExtensionMessageType.ProofProcessing,
        payload: {},
      });
    })
    .otherwise((message) => {
      console.error("No handler for message", message);
    });
});

browser.runtime.onMessageExternal.addListener(
  (message: MessageToExtension, sender) => {
    return match(message)
      .with(
        { action: ExtensionAction.RequestWebProof },
        async (msg) => await handleProofRequest(msg, sender),
      )
      .with(
        { action: ExtensionAction.NotifyZkProvingStatus },
        async (msg) => await handleProvingStatusNotification(msg),
      )
      .with(
        { action: ExtensionAction.OpenSidePanel },
        async () => await handleOpenSidePanel(sender),
      )
      .with({ action: ExtensionAction.CloseSidePanel }, () =>
        handleCloseSidePanel(),
      )
      .exhaustive();
  },
);

const handleOpenSidePanel = async (sender?: browser.Runtime.MessageSender) => {
  if (chrome.sidePanel && sender?.tab?.windowId) {
    await chrome.sidePanel.open({ windowId: sender.tab?.windowId });
  }
};

const handleCloseSidePanel = () => {
  void browser.runtime.sendMessage(ExtensionMessageType.CloseSidePanel);
};

const cleanProvingSessionStorageOnClose = () => {
  void browser.runtime.sendMessage(
    ExtensionMessageType.CleanProvingSessionStorageOnClose,
  );
};

const handleProofRequest = async (
  message: Extract<
    MessageToExtension,
    { action: ExtensionAction.RequestWebProof }
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
    { action: ExtensionAction.NotifyZkProvingStatus }
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
    { action: ExtensionAction.RequestWebProof }
  >,
) => {
  try {
    message.payload.steps.forEach(({ step, url }) => {
      match({ step, url })
        .with({ step: EXTENSION_STEP.startPage }, ({ url }) => assertUrl(url))
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
        type: ExtensionMessageType.SidePanelClosed,
        payload: {},
      });
    });
  }
});
