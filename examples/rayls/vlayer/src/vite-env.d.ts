/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_CHAIN_NAME: string;
  readonly VITE_PROVER_URL: string;
  readonly VITE_JSON_RPC_URL: string;
  readonly VITE_PRIVATE_KEY: `0x${string}`;
}
