import { getConfig } from "@vlayer/sdk/config";
import { createTestClient, http, publicActions, walletActions } from "viem";
import { foundry } from "viem/chains";
import MockERC20 from "../out/MockERC20.sol/MockERC20";
import { privateKeyToAccount } from "viem/accounts";

export const loadFixtures = async () => {
  const config = getConfig();

  const testClient = createTestClient({
    chain: foundry,
    mode: "anvil",
    transport: http(config.jsonRpcUrl),
  })
    .extend(publicActions)
    .extend(walletActions);

  const [john] = await testClient.getAddresses();

  const account = privateKeyToAccount(config.privateKey);

  const hash = await testClient.deployContract({
    abi: MockERC20.abi,
    bytecode: MockERC20.bytecode.object,
    account,
    args: ["Test", "TEST"],
  });

  const receipt = await testClient.waitForTransactionReceipt({ hash });
  const erc20addr = receipt.contractAddress as `0x${string}`;

  await testClient.writeContract({
    address: erc20addr,
    abi: MockERC20.abi,
    functionName: "mint",
    args: [account.address, 1000n],
    account,
  });
  await testClient.mine({ blocks: 10 });

  await testClient.writeContract({
    address: erc20addr,
    abi: MockERC20.abi,
    functionName: "transfer",
    args: [john, 100n],
    account,
  });
  await testClient.mine({ blocks: 10 });

  await testClient.writeContract({
    address: erc20addr,
    abi: MockERC20.abi,
    functionName: "transfer",
    args: [john, 100n],
    account,
  });
  await testClient.mine({ blocks: 20 });
};
