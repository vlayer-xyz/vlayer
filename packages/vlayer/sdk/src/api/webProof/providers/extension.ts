import {
  type WebProofProvider,
  type WebProofProviderSetup,
  type WebProofSetupInput,
} from "../../lib/types/webProofProvider";

import {
  ExtensionAction,
  ExtensionMessage,
} from "@vlayer/web-proof-commons/constants/message";

import { WebProof } from "../../lib/types/webProof";

// NOTE @types/chrome and webextension-polyfill work only in the extension context
// and looks that there is no community driven package providing typings for chrome.runtime
// or polyfill logic for the browser APIs available in the browser context
// we intentionally use chrome here instead of browser as we support only chrome for now
// and there could be some differences in the API between browsers

declare const chrome: {
  runtime: {
    sendMessage: (extensionId: string | undefined, message: unknown) => void;
    connect: (extensionId: string) => {
      onMessage: {
        addListener: (message: unknown) => void;
      };
    };
  };
};

export const createExtensionWebProofProvider = ({
  notaryUrl = "https://notary.pse.dev/v0.1.0-alpha.5/",
  wsProxyUrl = "wss://notary.pse.dev/proxy",
}: WebProofProviderSetup): WebProofProvider => {
  return {
    getWebProof: async function (webProofSetup: WebProofSetupInput) {
      // TODO: we can't assume that developer is using vite
      // VITE_EXTENSION_ID value should be injected by the build system

      return new Promise<WebProof>((resolve, reject) => {
        chrome.runtime.sendMessage(import.meta.env.VITE_EXTENSION_ID, {
          action: ExtensionAction.RequestWebProof,
          payload: {
            notaryUrl,
            wsProxyUrl,
            logoUrl: webProofSetup.logoUrl,
            steps: webProofSetup.steps,
          },
        });
        const EXTENSION_ID = import.meta.env.VITE_EXTENSION_ID as string;
        const port = chrome.runtime.connect(EXTENSION_ID);
        // TODO: validate message in runtime
        port.onMessage.addListener(
          (
            message:
              | {
                  type: ExtensionMessage.ProofDone;
                  proof: WebProof;
                }
              | {
                  type: ExtensionMessage.ProofError;
                  error: { message: string };
                },
          ) => {
            if (message.type === ExtensionMessage.ProofDone) {
              resolve(message.proof);
            }
            if (message.type === ExtensionMessage.ProofError) {
              reject(message.error);
            }
          },
        );
      });
    },
  };
};
