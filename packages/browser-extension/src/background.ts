import browser from "webextension-polyfill";

import {
  ExtensionAction,
  ExtensionMessage,
  ExtensionMessageType,
  MessageToExtension,
} from "./web-proof-commons";

import { WebProverSessionContextManager } from "./state/webProverSessionContext";
import { match, P } from "ts-pattern";
import { zkProvingStatusStore } from "./state/zkProvingStatusStore.ts";
import packageJson from "../package.json";

let port: browser.Runtime.Port | undefined = undefined;
let openedTabId: number | undefined = undefined;

browser.runtime.onConnectExternal.addListener((connectedPort) => {
  console.log("onConnectExternal", connectedPort);
  port = connectedPort;
  port.onMessage.addListener((message: MessageToExtension) => {
    match(message)
      .with({ action: ExtensionAction.RequestWebProof }, (msg) => {
        void handleProofRequest(msg, connectedPort.sender);
      })
      .with({ action: ExtensionAction.NotifyZkProvingStatus }, (msg) => {
        void handleProvingStatusNotification(msg);
      })
      .with({ action: ExtensionAction.RequestVersion }, () => {
        console.log("RequestVersion", packageJson.version);
        void handleVersionRequest(connectedPort);
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
        console.log("sending message to webpage", message);
        try {
          port?.postMessage(message);
        } catch (e) {
          console.log("Could not send message to webpage", e);
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
    .with({ type: ExtensionMessageType.Version }, () => {
      console.log("Version", packageJson.version);
    })
    .exhaustive();
});

browser.runtime.onMessageExternal.addListener(
  (message: MessageToExtension, sender) => {
    return match(message)
      .with({ action: ExtensionAction.RequestWebProof }, (msg) =>
        handleProofRequest(msg, sender),
      )
      .with({ action: ExtensionAction.NotifyZkProvingStatus }, (msg) =>
        handleProvingStatusNotification(msg),
      )
      .with({ action: ExtensionAction.RequestVersion }, () => {
        console.log("RequestVersion", packageJson.version);
      })
      .exhaustive();
  },
);

const handleProofRequest = async (
  message: Extract<
    MessageToExtension,
    { action: ExtensionAction.RequestWebProof }
  >,
  sender?: browser.Runtime.MessageSender,
) => {
  if (chrome.sidePanel && sender?.tab?.windowId) {
    await chrome.sidePanel.open({ windowId: sender.tab?.windowId });
  }
  await browser.storage.local.clear();
  await browser.storage.session.clear();
  await WebProverSessionContextManager.instance.setWebProverSessionConfig(
    message.payload,
  );
};

const handleProvingStatusNotification = async (
  message: Extract<
    MessageToExtension,
    { action: ExtensionAction.NotifyZkProvingStatus }
  >,
) => {
  await zkProvingStatusStore.setProvingStatus(message.payload);
};

const handleVersionRequest = (port: browser.Runtime.Port) => {
  port.postMessage({
    type: ExtensionMessageType.Version,
    payload: { version: packageJson.version },
  });
};
