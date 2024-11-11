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

let windowId = 0;
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

browser.tabs.onActivated.addListener(function (activeInfo) {
  windowId = activeInfo.windowId;
});

browser.tabs
  .query({ active: true, currentWindow: true })
  .then((tabs) => {
    windowId = tabs[0].windowId || 0;
  })
  .catch(console.error);

browser.runtime.onMessageExternal.addListener((message: MessageToExtension) => {
  (async () => {
    if (message.action === ExtensionAction.RequestWebProof) {
      if (chrome.sidePanel) {
        await chrome.sidePanel.open({ windowId: windowId });
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
});
