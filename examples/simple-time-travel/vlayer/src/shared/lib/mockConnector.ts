import { Chain } from "viem";
import { createConnector } from "wagmi";
import { getAccountFromPrivateKey } from "./clientAuthMode";

export const mockConnector = (chain: Chain) => {
  return createConnector((config) => ({
    ...config,
    id: "mock-connector",
    name: "Mock Connector",
    type: "mock",
    connect: async () => ({
      accounts: [getAccountFromPrivateKey().address],
      chainId: chain.id,
    }),
    disconnect: async () => {},
    getAccounts: async () => [getAccountFromPrivateKey().address],
    getChainId: async () => chain.id,
    getProvider: async () => ({}),
    isAuthorized: async () => true,
    address: getAccountFromPrivateKey().address,
    onAccountsChanged: () => {},
    onChainChanged: () => {},
    onDisconnect: () => {},
  }));
};