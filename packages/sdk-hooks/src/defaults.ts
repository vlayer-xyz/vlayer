import { ProofEnv } from "./types";

export const DEFAULT_CONFIG = {
  [ProofEnv.DEV]: {
    proverUrl: "http://localhost:3000",
    notaryUrl: "http://localhost:7047",
    wsProxyUrl: "ws://localhost:3003",
  },
  //for now we use the same urls for testnet and prod
  [ProofEnv.TESTNET]: {
    proverUrl: "https://stable-fake-prover.vlayer.xyz",
    notaryUrl: "https://test-notary.vlayer.xyz",
    wsProxyUrl: "wss://test-wsproxy.vlayer.xyz",
  },
  [ProofEnv.PROD]: {
    proverUrl: "https://stable-prod-prover.vlayer.xyz",
    notaryUrl: "https://test-notary.vlayer.xyz",
    wsProxyUrl: "wss://test-wsproxy.vlayer.xyz",
  },
};

export const DEFAULT_CONFIG_ENV = ProofEnv.TESTNET;
