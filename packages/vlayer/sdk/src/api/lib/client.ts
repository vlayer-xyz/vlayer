import { VlayerClient } from "types/vlayer";
import { WebProofProvider } from "types/webProofProvider";

import { prove } from "../prover";
import { createExtensionWebProofProvider } from "../webProof";
export const createVlayerClient = ({
  url = "127.0.0.1:3000",
  webProofProvider = createExtensionWebProofProvider(),
}: {
  url?: string;
  webProofProvider?: WebProofProvider;
} = {
  url: "127.0.0.1:3000",
  webProofProvider: createExtensionWebProofProvider(),
}): VlayerClient => {
  // TODO : implement high level api
  console.log("createVlayerClient with", url, webProofProvider);
  return {
    prove: async ({ address, functionName, chainId, proverAbi, args }) => {
      return prove(address, proverAbi, functionName, args, chainId);
    },
  };
};
