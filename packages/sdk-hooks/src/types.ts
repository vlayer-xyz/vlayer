import { type VlayerClient, type WebProofProvider } from "@vlayer/sdk";

export type VlayerContextType = {
  vlayerClient: VlayerClient;
  webProofProvider: WebProofProvider;
};
