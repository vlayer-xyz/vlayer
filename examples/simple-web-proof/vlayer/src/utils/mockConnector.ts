import { Chain } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { createConnector } from "wagmi";

const getAddressFromPrivateKey = () => {
  let address = "";
  const envPrivateKey = import.meta.env.VITE_PRIVATE_KEY;
  if (!envPrivateKey) {
    throw new Error("No private key found");
  } else {
    address = privateKeyToAccount(envPrivateKey as "0x").address;
  }
  return address as "0x";
};

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
