import { getConfig } from "@vlayer/sdk/config";
import {
  Chain,
  createTestClient,
  GetBlockReturnType,
  http,
  keccak256,
  publicActions,
  walletActions,
} from "viem";
import { foundry } from "viem/chains";
import MockERC20 from "../out/MockERC20.sol/MockERC20";
import { privateKeyToAccount } from "viem/accounts";
import L2State from "../out/L2State.sol/L2State";

const l1 = {
  ...foundry,
  id: 31_337,
};

const opL2 = {
  ...foundry,
  id: 31_338,
};

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

function computeOutputRoot(
  latestBlock: GetBlockReturnType<Chain, false, "latest">,
): `0x${string}` {
  const payload = [
    "00".repeat(32),
    latestBlock.stateRoot.slice(2),
    (latestBlock.withdrawalsRoot ?? `0x${"00".repeat(32)}`).slice(2),
    latestBlock.hash.slice(2),
  ].join("");

  return keccak256(`0x${payload}` as `0x${string}`);
}

export const l1TestClient = createAnvilClient(l1, config.jsonRpcUrl);
export const l2TestClient = createAnvilClient(opL2, config.l2JsonRpcUrl!);

const [john] = await l2TestClient.getAddresses();

const account = privateKeyToAccount(config.privateKey as `0x${string}`);

const hash = await l2TestClient.deployContract({
  abi: MockERC20.abi,
  bytecode: MockERC20.bytecode.object,
  account,
  args: ["L2 ERC20 Token", "L2ERC20"],
});

const receipt = await l2TestClient.waitForTransactionReceipt({ hash });

console.log("Contract deployed at:", receipt.contractAddress);
const erc20addr = receipt.contractAddress as `0x${string}`;

await l2TestClient.writeContract({
  address: erc20addr,
  abi: MockERC20.abi,
  functionName: "mint",
  args: [account.address, 1000n],
  account,
});
await l2TestClient.writeContract({
  address: erc20addr,
  abi: MockERC20.abi,
  functionName: "transfer",
  args: [john, 100n],
  account,
});

const latestBlock = await l2TestClient.getBlock({ blockTag: "latest" });
const outputRoot = computeOutputRoot(latestBlock);

await l1TestClient.deployContract({
  abi: L2State.abi,
  bytecode: L2State.bytecode.object,
  account,
  args: [outputRoot, latestBlock.number],
});
