import type {
  WebProofProvider,
  WebProofProviderSetup,
  WebProofSetupInput,
} from "../../lib/types/webProofProvider";

import { WebProof } from "../../lib/types/webProof";

declare const chrome: {
  runtime: {
    sendMessage: (extensionId: string | undefined, message: any) => void;
    connect: (extensionId: string) => any;
  };
};

export const createExtensionWebProofProvider = ({
  notaryUrl = "https://notary.pse.dev/v0.1.0-alpha.5/",
  wsProxyUrl = "wss://notary.pse.dev/proxy",
}: WebProofProviderSetup): WebProofProvider => {
  return {
    getWebProof: async function (webProofSetup: WebProofSetupInput) {
      //TODO: we cant assume that developer is using vite
      // EXTESION_ID value should be injected by the build system
      return new Promise<WebProof>((resolve, reject) => {
        chrome.runtime.sendMessage(import.meta.env.VITE_EXTENSION_ID, {
          action: "open_side_panel",
          payload: {
            notaryUrl,
            wsProxyUrl,
            logoUrl: webProofSetup.logoUrl,
            steps: webProofSetup.steps,
          },
        });
        const EXTENSION_ID = import.meta.env.VITE_EXTENSION_ID as string;
        const port = chrome.runtime.connect(EXTENSION_ID);
        port.onMessage.addListener(
          (message: { type: string; proof: WebProof }) => {
            if (message.type === "proof_done") {
              resolve(message.proof);
            }
          },
        );
      });
    },
  };
};
