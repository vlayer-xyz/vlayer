import { Page } from "@playwright/test";
import { installMockWallet } from "@johanneskares/wallet-mock";
import { privateKeyToAccount } from "viem/accounts";
import { http } from "viem";
import { anvil, optimismSepolia, sepolia } from "viem/chains";
import { getConfig } from "@vlayer/sdk/config";

const { privateKey } = getConfig();

export const useMockWallet = (page: Page) => {
  return installMockWallet({
    page,
    account: privateKeyToAccount(privateKey),
    transports: {
      [anvil.id]: http(),
      [sepolia.id]: http(),
      [optimismSepolia.id]: http(),
    },
  });
};
