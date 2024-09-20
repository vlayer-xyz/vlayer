import browser from "webextension-polyfill";
import { MESSAGE } from "./constants/message";

chrome.tabs.onActivated.addListener(function (activeInfo) {
  console.log("Actives tab changed", activeInfo);
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
    message.type === MESSAGE.proof_done ||
    message.type === MESSAGE.proof_error
  ) {
    try {
      port?.postMessage(message);
    } catch (e) {
      console.log("Could not send message to webpage", e);
    }
  }
});

let windowId = 0;
browser.tabs.onActivated.addListener(function (activeInfo) {
  windowId = activeInfo.windowId;
});

browser.runtime.onMessageExternal.addListener((message) => {
  (async () => {
    if (message.action === "open_side_panel") {
      //We nned to use chrome specific API to open side panel
      //as webextension-polyfill does not support it
      if (chrome.sidePanel) {
        chrome.sidePanel.open({ windowId: windowId });
      }
    }
  })();
});
