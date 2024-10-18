import { testHelpers } from "@vlayer/sdk";
import Bun from "bun";
import path from "node:path";
import fs from "node:fs/promises";
import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";
import dotenv from "dotenv";
import { createPublicClient, createWalletClient, http } from "viem";
import { optimismSepolia } from "viem/chains";
import { privateKeyToAccount } from "viem/accounts";

const envPath = path.resolve(__dirname, ".env.development");
dotenv.config({ path: envPath });

let prover, verifier;

if (process.env.VITE_TEST_PRIV_KEY) {
  const deployer = privateKeyToAccount(
    process.env.VITE_TEST_PRIV_KEY as `0x${string}`,
  );

  const walletClient = createWalletClient({
    chain: optimismSepolia,
    transport: http("https://sepolia.optimism.io"),
  });

  const client = createPublicClient({
    chain: optimismSepolia,
    transport: http("https://sepolia.optimism.io"),
  });
  console.log("Deploying Prover to OP Sepolia...");
  let txHash = await walletClient.deployContract({
    abi: webProofProver.abi,
    bytecode: webProofProver.bytecode.object,
    account: deployer,
    args: [],
    chain: optimismSepolia,
  });

  let receipt = await client.waitForTransactionReceipt({ hash: txHash });
  prover = receipt.contractAddress;
  console.log("Prover deployed OP Sepolia: ", prover);

  console.log("Deploying Verifier to OP Sepolia...");
  txHash = await walletClient.deployContract({
    abi: webProofVerifier.abi,
    bytecode: webProofVerifier.bytecode.object,
    account: deployer,
    args: [prover as `0x${string}`],
    chain: optimismSepolia,
  });

  receipt = await client.waitForTransactionReceipt({ hash: txHash });
  verifier = receipt.contractAddress;
  console.log("Verifier deployed OP Sepolia: ", verifier);
} else {
  [prover, verifier] = await testHelpers.deployProverVerifier(
    webProofProver,
    webProofVerifier,
  );
}

try {
  await fs.appendFile(envPath, "");
  const envFile = Bun.file(envPath);
  let envContent = await envFile.text();

  if (!envContent) {
    envContent = "";
  }

  const proverRegex = /^VITE_PROVER_ADDRESS=.*/m;
  const verifierRegex = /^VITE_VERIFIER_ADDRESS=.*/m;

  if (proverRegex.test(envContent) && prover) {
    envContent = envContent.replace(
      proverRegex,
      `VITE_PROVER_ADDRESS=${prover.trim()}`,
    );
  } else {
    envContent += `VITE_PROVER_ADDRESS=${prover}\n`;
  }

  if (verifierRegex.test(envContent)) {
    envContent = envContent.replace(
      verifierRegex,
      `VITE_VERIFIER_ADDRESS=${verifier}`,
    );
  } else {
    envContent += `VITE_VERIFIER_ADDRESS=${verifier}\n`;
  }

  await Bun.write(envPath, envContent);
  console.log("Successfully updated the .env.development file");
} catch (err) {
  console.error("Error updating the .env.development file:", err);
}
