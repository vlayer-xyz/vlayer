import browser from "webextension-polyfill";
import type { ExtensionInternalMessage } from "../web-proof-commons";

async function sendMessageToServiceWorker(message: ExtensionInternalMessage) {
  await browser.runtime.sendMessage(message);
}

export default sendMessageToServiceWorker;
