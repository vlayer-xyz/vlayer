import { optimismSepolia } from "viem/chains";
import {
  createPublicClient,
  createWalletClient,
  http,
  type Address,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";

import { createVlayerClient } from "@vlayer/sdk";
import averageBalance from "../out/AverageBalance.sol/AverageBalance";
import averageBalanceVerifier from "../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";

let privateKey = process.env.EXAMPLES_TEST_PRIVATE_KEY;

if (!privateKey) {
  throw new Error("EXAMPLES_TEST_PRIVATE_KEY environment variable is not set.");
}
privateKey = privateKey.startsWith("0x") ? privateKey : `0x${privateKey}`;
const deployer = privateKeyToAccount(privateKey as `0x${string}`);

const tokenOwner = "0xE6b08c02Dbf3a0a4D3763136285B85A9B492E391";

const walletClient = createWalletClient({
  chain: optimismSepolia,
  transport: http("https://sepolia.optimism.io"),
});

const client = createPublicClient({
  chain: optimismSepolia,
  transport: http("https://sepolia.optimism.io"),
});

const deployProver = async () => {
  const usdcTokenAddr = "0x5fd84259d66Cd46123540766Be93DFE6D43130D7"; // Test USDC at OP Sepolia
  const startBlock = 17915294n;
  const endBlock = 17985294n;
  const step = 9000n;

  const txHash = await walletClient.deployContract({
    abi: averageBalance.abi,
    bytecode: averageBalance.bytecode.object,
    account: deployer,
    args: [usdcTokenAddr, startBlock, endBlock, step],
    chain: optimismSepolia,
  });

  const receipt = await client.waitForTransactionReceipt({
    hash: txHash,
    confirmations: 5,
  });

  if (receipt.status != "success") {
    throw new Error(
      `Contract deployment failed with status: ${receipt.status}`,
    );
  }

  return receipt.contractAddress as Address;
};

const deployVerifier = async (prover: Address) => {
  const txHash = await walletClient.deployContract({
    abi: averageBalanceVerifier.abi,
    bytecode: averageBalanceVerifier.bytecode.object,
    account: deployer,
    args: [prover],
    chain: optimismSepolia,
  });

  const receipt = await client.waitForTransactionReceipt({
    hash: txHash,
    confirmations: 5,
  });

  if (receipt.status != "success") {
    throw new Error(
      `Contract deployment failed with status: ${receipt.status}`,
    );
  }

  return receipt.contractAddress as Address;
};

console.log("Deploying Prover...");
const proverAddr = await deployProver();
console.log(
  `Prover deployed: https://sepolia-optimism.etherscan.io/address/${proverAddr}`,
);

console.log("Deploying Verifier...");
const verifierAddr = await deployVerifier(proverAddr);
console.log(
  `Verifier deployed: https://sepolia-optimism.etherscan.io/address/${verifierAddr}`,
);

console.log("Proving...");
const vlayer = createVlayerClient();

const hash = await vlayer.prove({
  address: proverAddr,
  proverAbi: averageBalance.abi,
  functionName: "averageBalanceOf",
  args: [tokenOwner],
  chainId: optimismSepolia.id,
});
console.log("Waiting for proving result: ", hash);

const result = await vlayer.waitForProvingResult(hash);
console.log("Response:", result);

const txHash = await walletClient.writeContract({
  address: verifierAddr,
  abi: averageBalanceVerifier.abi,
  functionName: "claim",
  args: result,
  account: deployer,
});
console.log(
  `Verification tx: https://sepolia-optimism.etherscan.io/tx/${txHash}`,
);
const receipt = await client.waitForTransactionReceipt({
  hash: txHash,
  confirmations: 5,
});

console.log(`Verification result: ${receipt.status}`);
