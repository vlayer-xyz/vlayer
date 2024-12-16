import { type VlayerClient, type WebProofProvider } from "@vlayer/sdk";
import { type VlayerChainContext } from "@vlayer/sdk/config";

export type VlayerContextType = {
  vlayerClient: VlayerClient;
  webProofProvider: WebProofProvider;
  chainContext: VlayerChainContext;
};
