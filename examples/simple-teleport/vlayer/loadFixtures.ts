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
import { type Address } from "viem";

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
    (latestBlock.withdrawalsRoot ?? `0x`).slice(2),
    latestBlock.hash.slice(2),
  ].join("");

  return keccak256(`0x${payload}` as Address);
}

export const l1TestClient = createAnvilClient(l1, config.jsonRpcUrl);
export const l2TestClient = createAnvilClient(opL2, config.l2JsonRpcUrl!);

const account = privateKeyToAccount(config.privateKey as Address);

const opAccount = privateKeyToAccount(
  process.env.EXAMPLES_TEST_OP_PRIVATE_KEY as Address,
);

const hash = await l2TestClient.deployContract({
  abi: MockERC20.abi,
  bytecode: MockERC20.bytecode.object,
  account: opAccount,
  args: ["L2 ERC20 Token", "L2ERC20"],
});

const receipt = await l2TestClient.waitForTransactionReceipt({ hash });

const erc20addr = receipt.contractAddress as `0x${string}`;

await l2TestClient.writeContract({
  address: erc20addr,
  abi: MockERC20.abi,
  functionName: "mint",
  args: [opAccount.address, 1000n],
  account: opAccount,
});
await l2TestClient.writeContract({
  address: erc20addr,
  abi: MockERC20.abi,
  functionName: "transfer",
  args: [process.env.TOKEN_HOLDER as Address, 100n],
  account: opAccount,
});

const latestBlock = await l2TestClient.getBlock({ blockTag: "latest" });
const outputRoot = computeOutputRoot(latestBlock);

await l1TestClient.deployContract({
  abi: L2State.abi,
  bytecode: L2State.bytecode.object,
  account,
  args: [outputRoot, latestBlock.number],
});
