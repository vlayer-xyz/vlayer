import browser from "webextension-polyfill";
import {
  ExtensionAction,
  ExtensionMessageType,
} from "@vlayer/web-proof-commons";
import { WebProverSessionContextManager } from "./state/webProverSessionContext";

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

browser.runtime.onMessage.addListener(async (message) => {
  if (
    message.type === ExtensionMessageType.ProofDone ||
    message.type === ExtensionMessageType.ProofError
  ) {
    console.log("sending message to webpage", message);
    try {
      port?.postMessage(message);
    } catch (e) {
      console.log("Could not send message to webpage", e);
    }
  }
  console.log(message);
  if (message.type === ExtensionMessageType.RedirectBack) {
    console.log("Redirect back to webpage", port?.sender?.tab?.id);
    //close current
    const currentTab = (await browser.tabs.query({ active: true }))[0];
    await (currentTab.id && currentTab.id !== port?.sender?.tab?.id
      ? browser.tabs.remove(currentTab?.id)
      : Promise.resolve());
    browser.tabs.update(port?.sender?.tab?.id, { active: true });
  }
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
