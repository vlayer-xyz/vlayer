import browser from "webextension-polyfill";

console.log("Hello from the background!");

browser.runtime.onInstalled.addListener((details) => {
  console.log("Extension installed:", details);
});

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

let resolveProofPromise = new Promise(() => {});

browser.runtime.onMessageExternal.addListener((message, sender, sendResponse) => {
  console.log("Received message from external sender", message.type);
  
  if (message.type === "REQUEST_WEB_PROOF") {
    console.log("got request for web proof");
    console.log("sending message to content script",sender.tab?.id);
    const orginalTabId = sender.tab?.id;
      // sendResponse({ type: "PROOF", proof: "proof" });

      browser.runtime.onMessage.addListener((message, sender) => {

      if (message.type === "PROOF") {
        console.log("Received PROOF message from content script", message);
        //@ts-ignore
        console.log("sending message to original tab", orginalTabId);
        browser.tabs.sendMessage(orginalTabId || 0, { type: "PROOF_RECEIVED" });
      }
    });
  }
});
