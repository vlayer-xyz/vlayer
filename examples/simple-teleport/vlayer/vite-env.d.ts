interface ImportMetaEnv {
  readonly PROVER_START_BLOCK: bigint | "latest";
  readonly PROVER_END_BLOCK: bigint;
  readonly PROVER_STEP_BACK: bigint;
  readonly PROVER_ERC20_CONTRACT_ADDR: string;
  readonly PROVER_ERC20_HOLDER_ADDR: string;
  // more env variables...
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
