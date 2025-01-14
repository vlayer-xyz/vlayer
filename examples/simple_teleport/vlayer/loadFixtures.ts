import { getConfig } from "@vlayer/sdk/config";
import { Chain, createTestClient, http, publicActions, walletActions } from "viem";
import { foundry } from "viem/chains";
import MockERC20 from "../out/MockERC20.sol/MockERC20";
import { privateKeyToAccount } from "viem/accounts";

let l1 = {
  ...foundry,
  id: 31_337,
};

let opL2 = {
  ...foundry,
  id: 31_338,
}

const config = getConfig();

function createAnvilClient(chain: Chain, url: string) {
  return createTestClient({
    chain,
    mode: "anvil",
    transport: http(url),
  })
    .extend(publicActions)
    .extend(walletActions);
}

export const l1TestClient = createAnvilClient(l1, config.jsonRpcUrl);
export const l2TestClient = createAnvilClient(opL2, config.l2JsonRpcUrl!);

const [john] = await l1TestClient.getAddresses();

const account = privateKeyToAccount(config.privateKey as `0x${string}`);

const hash = await l1TestClient.deployContract({
  abi: MockERC20.abi,
  bytecode: MockERC20.bytecode.object,
  account,
  args: ["L2 ERC20 Token", "L2ERC20"],
});

const receipt = await l1TestClient.waitForTransactionReceipt({ hash });
const erc20addr = receipt.contractAddress as `0x${string}`;

await l1TestClient.writeContract({
  address: erc20addr,
  abi: MockERC20.abi,
  functionName: "mint",
  args: [account.address, 1000n],
  account,
});
await l1TestClient.writeContract({
  address: erc20addr,
  abi: MockERC20.abi,
  functionName: "transfer",
  args: [john, 100n],
  account,
});
await l1TestClient.mine({ blocks: 1 });
