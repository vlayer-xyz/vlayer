/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_SENTRY_DSN: string;
  readonly VITE_PROVER_URL: string;
  readonly VITE_NOTARY_URL: string;
  readonly VITE_WS_PROXY_URL: string;
  readonly VITE_VLAYER_API_TOKEN: string;
  readonly VITE_CHAIN_NAME: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
