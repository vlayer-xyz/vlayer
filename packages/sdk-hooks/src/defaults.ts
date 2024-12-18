import { ProofEnv } from "./types";

export const DEFAULT_CONFIG = {
  dev: {
    proverUrl: "http://localhost:3000",
    notaryUrl: "http://localhost:7047",
    wsProxyUrl: "ws://localhost:55688",
  },
  //for now we use the same urls for testnet and prod
  testnet: {
    proverUrl: "https://test-prover.vlayer.xyz",
    notaryUrl: "https://test-notary.vlayer.xyz",
    wsProxyUrl: "wss://test-wsproxy.vlayer.xyz",
  },
  prod: {
    proverUrl: "https://test-prover.vlayer.xyz",
    notaryUrl: "https://test-notary.vlayer.xyz",
    wsProxyUrl: "wss://test-wsproxy.vlayer.xyz",
  },
};

export const DEFAULT_CONFIG_ENV = ProofEnv.DEV;
