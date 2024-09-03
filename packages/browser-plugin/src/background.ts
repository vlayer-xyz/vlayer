import browser from "webextension-polyfill";
import * as vlayerSDK from "@vlayer/sdk"; 

console.log("Hello from the background!",vlayerSDK);

browser.runtime.onInstalled.addListener((details) => {
  console.log("Extension installed:", details);
});

