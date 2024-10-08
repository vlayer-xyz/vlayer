import { optimismSepolia } from "viem/chains";
import { createPublicClient, createWalletClient, http } from "viem";
import { privateKeyToAccount } from "viem/accounts";

import { createVlayerClient } from "@vlayer/sdk";
import averageBalance from "../out/AverageBalance.sol/AverageBalance";
import averageBalanceVerifier from "../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";

const chainId = optimismSepolia.id;
const tokenOwner = "0xE6b08c02Dbf3a0a4D3763136285B85A9B492E391";

console.log("Proving...");
const vlayer = createVlayerClient();

const { hash } = await vlayer.prove({
  address: "0x32A68f66789B12c96f1e2486D73bd33f4070F49b",
  proverAbi: averageBalance.abi,
  functionName: "averageBalanceOf",
  args: [tokenOwner],
  chainId,
});
const { proof, result } = await vlayer.waitForProvingResult({ hash });
console.log("Response:", proof, result);

const walletClient = createWalletClient({
  chain: optimismSepolia,
  transport: http("https://sepolia.optimism.io"),
});

const client = createPublicClient({
  chain: optimismSepolia,
  transport: http("https://sepolia.optimism.io"),
});

const privateKey = process.env.DEPLOYER_PRIV_KEY;

if (!privateKey) {
  throw new Error("DEPLOYER_PRIV_KEY environment variable is not set.");
}
const deployer = privateKeyToAccount(`0x${privateKey}`);

const txHash = await walletClient.writeContract({
  address: "0x0fb5a31f0c3155911bc30be7d0867337a1bfe483",
  abi: averageBalanceVerifier.abi,
  functionName: "claim",
  args: [proof, ...result],
  account: deployer,
});

const receipt = await client.waitForTransactionReceipt({ hash: txHash });

console.log(`Verification result: ${receipt.status}`);
