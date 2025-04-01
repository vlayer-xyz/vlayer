import { Page } from "@playwright/test";
import { installMockWallet } from "@johanneskares/wallet-mock";
import { privateKeyToAccount } from "viem/accounts";
import { http } from "viem";
import { anvil, optimismSepolia } from "viem/chains";
import { getConfig } from "@vlayer/sdk/config";

const { privateKey, chainName } = getConfig();
const chain = chainName === "anvil" ? anvil : optimismSepolia;

export const useMockWallet = (page: Page) => {
  return installMockWallet({
    page,
    account: privateKeyToAccount(privateKey),
    defaultChain: chain,
    transports: { [chain.id]: http() },
  });
};
