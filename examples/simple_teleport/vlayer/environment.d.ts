declare global {
  namespace NodeJS {
    interface ProcessEnv {
      PROVER_ERC20_ADDRESSES: string;
      PROVER_ERC20_CHAIN_IDS: string;
      PROVER_ERC20_BLOCK_NUMBERS: string;
    }
  }
}

export {};
