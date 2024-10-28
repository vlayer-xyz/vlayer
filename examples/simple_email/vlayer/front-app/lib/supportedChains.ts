import {
  mainnet,
  sepolia,
  base,
  baseSepolia,
  optimism,
  optimismSepolia,
  polygon,
  polygonAmoy,
  arbitrum,
  arbitrumNova,
  arbitrumSepolia,
  zksync,
  zksyncSepoliaTestnet,
} from "viem/chains";

import type { Chain } from "viem/chains";

const supportedChains = {
  [optimismSepolia.id]: optimismSepolia,
  [mainnet.id]: mainnet,
  [sepolia.id]: sepolia,
  [base.id]: base,
  [baseSepolia.id]: baseSepolia,
  [optimism.id]: optimism,
  [polygon.id]: polygon,
  [polygonAmoy.id]: polygonAmoy,
  [arbitrum.id]: arbitrum,
  [arbitrumNova.id]: arbitrumNova,
  [arbitrumSepolia.id]: arbitrumSepolia,
  [zksync.id]: zksync,
  [zksyncSepoliaTestnet.id]: zksyncSepoliaTestnet,
};
export const getChainConfig = (chainId: number): Chain => {
  const chain = supportedChains[chainId as keyof typeof supportedChains];

  if (!chain) throw new Error(`given chainId (${chainId}) is not supported`);

  return chain;
};
