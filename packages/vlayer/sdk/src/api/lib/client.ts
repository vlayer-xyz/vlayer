import { VlayerClient } from "types/vlayer";
import { WebProofProvider } from "types/webProofProvider";

import { prove } from "../prover";
export const createVlayerClient = ({
  url,
  webProofProvider,
}: {
  url: string;
  webProofProvider: WebProofProvider;
}): VlayerClient => {
  // TODO : implement high level api
  console.log("createVlayerClient with", url, webProofProvider);
  return {
    prove: async ({ address, functionName, chainId, proverAbi, args }) => {
      return prove(address, proverAbi, functionName, args, chainId, url);
    },
  };
};
