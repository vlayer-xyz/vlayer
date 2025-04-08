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
import { getTeleportConfig } from "./constants";
import { env } from "./env";

const l1 = {
  ...foundry,
  id: 31_337,
};

const opL2 = {
  ...foundry,
  id: 31_338,
};

// Anvil test private key, for the purpose of fixtures only.
const opAccount = privateKeyToAccount(
  "0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659" as Address,
);

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
  console.log(`State root: ${latestBlock.stateRoot}`);
  console.log(`Withdrawals root: ${latestBlock.withdrawalsRoot}`);
  console.log(`Block hash: ${latestBlock.hash}`);

  const payload = [
    "00".repeat(32),
    latestBlock.stateRoot.slice(2),
    (latestBlock.withdrawalsRoot ?? `0x`).slice(2),
    latestBlock.hash.slice(2),
  ].join("");

  const hash = keccak256(`0x${payload}`);
  console.log(`Payload hash: ${hash}`);
  return hash;
}

export const loadFixtures = async () => {
  const config = getConfig();
  const teleportConfig = getTeleportConfig(config.chainName);

  const l1TestClient = createAnvilClient(l1, config.jsonRpcUrl);
  const l2TestClient = createAnvilClient(opL2, env.L2_JSON_RPC_URL);
  const account = privateKeyToAccount(config.privateKey);

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
    args: [teleportConfig.tokenHolder, 100n],
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
};
