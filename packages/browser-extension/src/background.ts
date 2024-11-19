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

let port: browser.Runtime.Port | undefined = undefined;
let openedTabId: number | undefined = undefined;

browser.runtime.onInstalled.addListener((details) => {
  console.log("Extension installed:", details);
});

browser.runtime.onConnectExternal.addListener((connectedPort) => {
  port = connectedPort;
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
    .with({ type: ExtensionMessageType.TabOpened }, ({ tabId }) => {
      openedTabId = tabId;
    })
    .exhaustive();
});

browser.runtime.onMessageExternal.addListener(
  (message: MessageToExtension, sender) => {
    (async () => {
      if (message.action === ExtensionAction.RequestWebProof) {
        // only open side panel if it is supported and sender is browser tab
        if (chrome.sidePanel && sender.tab?.windowId) {
          await chrome.sidePanel.open({ windowId: sender.tab?.windowId });
        }
        await browser.storage.local.clear();
        await browser.storage.session.clear();
        await WebProverSessionContextManager.instance.setWebProverSessionConfig(
          message.payload,
        );
      } else if (message.action === ExtensionAction.NotifyZkProvingStatus) {
        console.log("Received proving status", message.payload);
        await zkProvingStatusStore.setProvingStatus(message.payload);
      }
    })().catch(console.error);
  },
);
