import { VlayerClient } from "types/vlayer";
import { WebProof, WebProofProvider } from "types/webProof";

export const creaateVlayerClient = ({
  url,
  webProofProvider,
}: {
  url: string;
  webProofProvider: WebProofProvider;
}): VlayerClient => {
  return {
    prove: async () => {},
  };
};
