/// <reference types="vite/client" />

type Address = "0x${string}";

interface ImportMetaEnv {
  readonly VITE_PROVER_ADDR: Address;
  readonly VITE_VERIFIER_ADDR: Address;
  readonly VITE_PROVER_URL: string;
  readonly VITE_EMAIL_SERVICE_URL: string;
  readonly VITE_VERIFIER_ADDRESS: Address;
  readonly VITE_PROVER_ADDRESS: Address;
  readonly VITE_DNS_SERVICE_URL: string;
  readonly VITE_VLAYER_API_TOKEN: string;
  readonly VITE_SENTRY_DSN: string;
  readonly VITE_CHAIN_NAME: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
