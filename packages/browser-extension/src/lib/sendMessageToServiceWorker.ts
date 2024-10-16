import browser from "webextension-polyfill";
import type { ExtensionMessage } from "../web-proof-commons";

async function sendMessageToServiceWorker(message: ExtensionMessage) {
  await browser.runtime.sendMessage(message);
}

export default sendMessageToServiceWorker;
