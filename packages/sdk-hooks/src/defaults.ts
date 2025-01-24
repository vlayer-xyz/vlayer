import { ProofEnv } from "./types";

export const DEFAULT_CONFIG = {
  [ProofEnv.DEV]: {
    proverUrl: "http://localhost:3000",
    notaryUrl: "http://localhost:7047",
    wsProxyUrl: "ws://localhost:55688",
  },
  //for now we use the same urls for testnet and prod
  [ProofEnv.TESTNET]: {
    proverUrl: "https://test-prover.vlayer.xyz",
    notaryUrl: "https://notary.pse.dev/v0.1.0-alpha.7",
    wsProxyUrl: "wss://test-wsproxy.vlayer.xyz",
  },
  [ProofEnv.PROD]: {
    proverUrl: "https://test-prover.vlayer.xyz",
    notaryUrl: "https://notary.pse.dev/v0.1.0-alpha.7",
    wsProxyUrl: "wss://test-wsproxy.vlayer.xyz",
  },
};

export const DEFAULT_CONFIG_ENV = ProofEnv.TESTNET;
