import { VlayerClient } from "types/vlayer";
import { WebProofProvider } from "types/webProof";

export const createVlayerClient = ({
  url,
  webProofProvider,
}: {
  url: string;
  webProofProvider: WebProofProvider;
}): VlayerClient => {
  return {
    prove: async () => {
      console.log("prove");
      console.log("url", url);
      console.log("webProofProvider", webProofProvider);
    },
  };
};
