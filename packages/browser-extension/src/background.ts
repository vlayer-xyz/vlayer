import browser from "webextension-polyfill";
import { MESSAGE } from "./constants/message";

let port : browser.Runtime.Port | undefined = undefined

browser.runtime.onInstalled.addListener((details) => {
  console.log("Extension installed:", details);
});

browser.runtime.onConnectExternal.addListener((connectedPort) => {
  console.log("Connected to external port", connectedPort);
  port = connectedPort; 
});

browser.runtime.onMessageExternal.addListener((message) => {
  //  for now we only work with connection request 
  // and we use hardcoded twitter 
  // in the future we will read message here and create proper execution context based 
  // on the payload 
});

browser.runtime.onMessage.addListener((message) => {
  if ( message.type === MESSAGE.proof_done  || message.type === MESSAGE.proof_error) {
    try {
      port?.postMessage(message);
    } catch (e) {
      console.log("Could not send message to webpage", port);
    }
  }
});