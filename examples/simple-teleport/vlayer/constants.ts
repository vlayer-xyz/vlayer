export interface TeleportConfig {
  tokenHolder: `0x${string}`;
  prover: {
    erc20Addresses: string;
    erc20ChainIds: string;
    erc20BlockNumbers: string;
  };
}

export const chainToTeleportConfig: Record<string, TeleportConfig> = {
  anvil: {
    tokenHolder: "0xe2148eE53c0755215Df69b2616E552154EdC584f",
    prover: {
      erc20Addresses: "0xda52b25ddB0e3B9CC393b0690Ac62245Ac772527",
      erc20ChainIds: "31338",
      erc20BlockNumbers: "3",
    },
  },
  sepolia: {
    tokenHolder: "0x4631d3E5803332448e0D9cBb9bF501A4C50B95ed",
    prover: {
      erc20Addresses: "0xc6e1fb449b08b26b2063c289df9bbcb79b91c992",
      erc20ChainIds: "11155420",
      erc20BlockNumbers: "25181931",
    },
  },
  mainnet: {
    tokenHolder: "0xacD03D601e5bB1B275Bb94076fF46ED9D753435A",
    prover: {
      erc20Addresses: "0x0b2c639c533813f4aa9d7837caf62653d097ff85",
      erc20ChainIds: "10",
      erc20BlockNumbers: "135459541",
    },
  },
};

export const getTeleportConfig = (chainName: string): TeleportConfig => {
  const config: TeleportConfig | undefined = chainToTeleportConfig[chainName];
  if (!config) {
    throw new Error(
      `The "${chainName}" chain is not yet configured in this example.`,
    );
  }
  return config;
};
