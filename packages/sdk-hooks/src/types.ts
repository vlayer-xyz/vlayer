import { type VlayerClient, type WebProofProvider } from "@vlayer/sdk";
import { expectUrl, notarize, startPage } from "@vlayer/sdk/web_proof";
import { type WebProofRequestInput, type WebProofConfig } from "@vlayer/sdk";

export type ProofContextType = {
  vlayerClient: VlayerClient;
  webProofProvider: WebProofProvider;
  config: ProofConfig;
};

export type WebProofContextType = {
  webProofProvider: WebProofProvider;
  config: Pick<ProofConfig, "notaryUrl" | "wsProxyUrl">;
};

export type ProverContextType = {
  vlayerClient: VlayerClient;
  config: Pick<ProofConfig, "proverUrl">;
};

export enum ProofEnv {
  DEV = "dev",
  TESTNET = "testnet",
  PROD = "prod",
}

export type ProofConfig = {
  proverUrl: string;
  notaryUrl: string;
  wsProxyUrl: string;
  env?: ProofEnv;
  token?: string;
};

export enum WebProofRequestStatus {
  idle = "idle",
  pending = "pending",
  error = "error",
  success = "success",
}

export {
  expectUrl,
  notarize,
  startPage,
  type WebProofRequestInput,
  type WebProofConfig,
};
