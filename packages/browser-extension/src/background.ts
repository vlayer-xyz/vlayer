import browser from "webextension-polyfill";
import {
  ExtensionAction,
  ExtensionMessage,
} from "@vlayer/web-proof-commons/constants/message";
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
  console.log("Connected to external port", connectedPort);
  port = connectedPort;
});

browser.runtime.onMessageExternal.addListener(() => {
  //  for now we only work with connection request
  // and we use hardcoded twitter
  // in the future we will read message here and create proper execution context based
  // on the payload
});

browser.runtime.onMessage.addListener((message) => {
  if (
    message.type === ExtensionMessage.ProofDone ||
    message.type === ExtensionMessage.ProofError
  ) {
    try {
      port?.postMessage(message);
    } catch (e) {
      console.log("Could not send message to webpage", e);
    }
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
        chrome.sidePanel.open({ windowId: windowId });
      }
      browser.storage.local.clear();
      await WebProverSessionContextManager.instance.setWebProverSessionConfig(
        message.payload,
      );
    }
  })();
});
