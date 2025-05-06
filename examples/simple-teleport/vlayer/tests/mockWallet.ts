import { Page } from "@playwright/test";
import { installMockWallet } from "@johanneskares/wallet-mock";
import { privateKeyToAccount } from "viem/accounts";
import { http } from "viem";
import { anvil, optimismSepolia, sepolia } from "viem/chains";
import { getConfig } from "@vlayer/sdk/config";

const { privateKey, chainName } = getConfig();
let chain;
switch (chainName) {
  case "anvil":
    chain = anvil;
    break;
  case "sepolia":
    chain = sepolia;
    break;
  case "optimismSepolia":
    chain = optimismSepolia;
    break;
  default:
    chain = optimismSepolia;
}

export const useMockWallet = (page: Page) => {
  return installMockWallet({
    page,
    account: privateKeyToAccount(privateKey),
    defaultChain: chain,
    transports: { [chain.id]: http() },
  });
};
