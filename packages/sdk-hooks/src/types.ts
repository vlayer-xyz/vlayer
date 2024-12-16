import { type VlayerClient, type WebProofProvider } from "@vlayer/sdk";

export type VlayerContextType = {
  vlayerClient: VlayerClient;
  webProofProvider: WebProofProvider;
};

export enum WebProofRequestStatus {
  idle = "idle",
  pending = "pending",
  error = "error",
  success = "success",
}
