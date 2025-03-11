import { Chain } from "viem";
import { createConnector } from "wagmi";
import { getAddressFromPrivateKey } from "./clientAuthMode";

export const mockConnector = (chain: Chain) => {
  return createConnector((config) => ({
    ...config,
    id: "mock-connector",
    name: "Mock Connector",
    type: "mock",
    connect: async () => ({
      accounts: [getAddressFromPrivateKey()],
      chainId: chain.id,
    }),
    disconnect: async () => {},
    getAccounts: async () => [getAddressFromPrivateKey()],
    getChainId: async () => chain.id,
    getProvider: async () => ({}),
    isAuthorized: async () => true,
    address: getAddressFromPrivateKey(),
    onAccountsChanged: () => {},
    onChainChanged: () => {},
    onDisconnect: () => {},
  }));
};
