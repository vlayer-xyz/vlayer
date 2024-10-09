import {
  type WebProofProvider,
  type WebProofProviderSetup,
  type WebProofSetupInput,
} from "../../lib/types/webProofProvider";

import {
  ExtensionAction,
  type ExtensionMessage,
  ExtensionMessageType,
  type MessageToExtension,
} from "@vlayer/web-proof-commons";

import { WebProof } from "../../lib/types/webProof";

// NOTE @types/chrome and webextension-polyfill work only in the extension context
// and looks that there is no community driven package providing typings for chrome.runtime
// or polyfill logic for the browser APIs available in the browser context
// we intentionally use chrome here instead of browser as we support only chrome for now
// and there could be some differences in the API between browsers

declare const chrome: {
  runtime: {
    sendMessage: (extensionId: string | undefined, message: MessageToExtension) => void;
    connect: (extensionId: string) => {
      onMessage: {
        addListener: (message: unknown) => void;
      };
    };
  };
};

// this id is fixed in the extension by the key in manifest.json
const EXTENSION_ID = "ghigbilfcgeibjkkajaekabeldkmijcd";

export const createExtensionWebProofProvider = (
  {
    notaryUrl = "https://notary.pse.dev/v0.1.0-alpha.5/",
    wsProxyUrl = "wss://notary.pse.dev/proxy",
  }: WebProofProviderSetup = {
    notaryUrl: "https://notary.pse.dev/v0.1.0-alpha.5/",
    wsProxyUrl: "wss://notary.pse.dev/proxy",
  },
): WebProofProvider => {
  return {
    getWebProof: async function (webProofSetup: WebProofSetupInput) {
      return new Promise<WebProof>((resolve, reject) => {
        chrome.runtime.sendMessage(EXTENSION_ID, {
          action: ExtensionAction.RequestWebProof,
          payload: {
            notaryUrl,
            wsProxyUrl,
            logoUrl: webProofSetup.logoUrl,
            steps: webProofSetup.steps,
          },
        });
        const port = chrome.runtime.connect(EXTENSION_ID);
        // TODO: validate message in runtime
        port.onMessage.addListener((message: ExtensionMessage) => {
          if (message.type === ExtensionMessageType.ProofDone) {
            resolve(message.proof);
          }
          if (message.type === ExtensionMessageType.ProofError) {
            reject(message.error);
          }
        });
      });
    },
  };
};
