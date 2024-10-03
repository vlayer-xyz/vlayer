import browser from "webextension-polyfill";
import {
  EXTENSION_ACTION,
  EXTENSION_MESSAGE_TYPE,
} from "@vlayer/web-proof-commons/constants/message";

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
    message.type === EXTENSION_MESSAGE_TYPE.proofDone ||
    message.type === EXTENSION_MESSAGE_TYPE.proofError
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
  console.log("Active tab changed", activeInfo);
  windowId = activeInfo.windowId;
});

browser.runtime.onMessageExternal.addListener((message) => {
  (async () => {
    if (message.action === EXTENSION_ACTION.requestWebProof) {
      if (chrome.sidePanel) {
        chrome.sidePanel.open({ windowId: windowId });
      }
    }
  })();
});
