export interface ChainConfig {
  usdcTokenAddr: `0x${string}`;
  tokenOwner: `0x${string}`;
  prover:
    | { endBlock: "latest"; travelRange: bigint; step: bigint }
    | { startBlock: bigint; endBlock: bigint; step: bigint };
}

export const chainConfigs: Record<string, ChainConfig> = {
  anvil: {
    tokenOwner: "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
    usdcTokenAddr: "0x5fbdb2315678afecb367f032d93f642f64180aa3",
    prover: {
      startBlock: BigInt(1),
      endBlock: BigInt(40),
      step: BigInt(10),
    },
  },
  optimismSepolia: {
    tokenOwner: "0x5fd84259d66Cd46123540766Be93DFE6D43130D7",
    usdcTokenAddr: "0x5fd84259d66Cd46123540766Be93DFE6D43130D7",
    prover: {
      endBlock: "latest",
      travelRange: BigInt(10),
      step: BigInt(2),
    },
  },
};

export const getChainConfig = (chainName: string): ChainConfig => {
  if (chainName === "anvil") {
    return chainConfigs.anvil;
  } else if (chainName == "optimismSepolia") {
    return chainConfigs.optimismSepolia;
  } else {
    throw new Error(
      `The "${chainName}" chain is not yet configured in this example.`,
    );
  }
};
