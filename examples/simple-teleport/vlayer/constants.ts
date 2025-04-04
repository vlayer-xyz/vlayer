export interface ChainConfig {
  tokenHolder: `0x${string}`;
  prover: {
    erc20Addresses: string;
    erc20ChainIds: string;
    erc20BlockNumbers: string;
  }
}

export const chainConfigs: Record<string, ChainConfig> = {
  anvil: {
    tokenHolder: '0xe2148eE53c0755215Df69b2616E552154EdC584f',
    prover: {
      erc20Addresses: '0xda52b25ddB0e3B9CC393b0690Ac62245Ac772527',
      erc20ChainIds: '31338',
      erc20BlockNumbers: '3',
    },
  },
  optimismSepolia: {
    tokenHolder: '0xA6E3d943197A53C5608fF49239310C19843B3Cf1',
    prover: {
      erc20Addresses: '0x298b4c4f9be251c100724a3beae234bd1652cbce',
      erc20ChainIds: '11155420',
      erc20BlockNumbers: '24192026',
    },
  },
};

export const getChainConfig = (chainName: string): ChainConfig => {
  const config: ChainConfig | undefined = chainConfigs[chainName];
  if (!config) {
    throw new Error(
      `The "${chainName}" chain is not yet configured in this example.`,
    );
  }
  return config;
};
