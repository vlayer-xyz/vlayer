/// <reference types="vite/client" />

type Address = "0x${string}";

interface ImportMetaEnv {
  readonly VITE_PROVER_ADDR: Address;
  readonly VITE_VERIFIER_ADDR: Address;
  readonly VITE_PROVER_URL: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
