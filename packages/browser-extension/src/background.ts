import browser from "webextension-polyfill";
import {
  ExtensionAction,
  ExtensionMessage,
  ExtensionMessageType,
} from "@vlayer/web-proof-commons";
import { WebProverSessionContextManager } from "./state/webProverSessionContext";
import { match, P } from "ts-pattern";

chrome.tabs.onActivated.addListener(function (activeInfo) {
  windowId = activeInfo.windowId;
});

// to receive messages from popup script
let port: browser.Runtime.Port | undefined = undefined;

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
      console.log("Redirect back to webpage", port?.sender?.tab?.id);
      //close current
      const currentTab = (await browser.tabs.query({ active: true }))[0];
      await (currentTab.id && currentTab.id !== port?.sender?.tab?.id
        ? browser.tabs.remove(currentTab?.id)
        : Promise.resolve());
      await browser.tabs.update(port?.sender?.tab?.id, { active: true });
    })
    .exhaustive();
});

let windowId = 0;

browser.tabs.query({ active: true, currentWindow: true }).then((tabs) => {
  windowId = tabs[0].windowId || 0;
});

browser.tabs.onActivated.addListener(function (activeInfo) {
  windowId = activeInfo.windowId;
});

browser.runtime.onMessageExternal.addListener((message) => {
  (async () => {
    if (message.action === ExtensionAction.RequestWebProof) {
      if (chrome.sidePanel) {
        await chrome.sidePanel.open({ windowId: windowId });
      }
      //TODO make cleanup logic separated method
      await browser.storage.local.clear();
      await browser.storage.session.clear();
      await WebProverSessionContextManager.instance.setWebProverSessionConfig(
        message.payload,
      );
    }
  })();
});
