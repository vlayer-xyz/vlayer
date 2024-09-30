import type {
  WebProofProvider,
  WebProofProviderSetup,
  WebProofSetupInput,
} from "../../lib/types/webProofProvider";

import { WebProof } from "../../lib/types/webProof";

export const createExtensionWebProofProvider = ({
  notaryUrl = "",
  wsProxyUrl = "",
}: WebProofProviderSetup): WebProofProvider => {
  return {
    getWebProof: async function (webProofSetup: WebProofSetupInput) {
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
            console.log("Message from extension", message);
            if (message.type === "proof_done") {
              resolve(message.proof);
            }
          },
        );
      });
    },
  };
};
