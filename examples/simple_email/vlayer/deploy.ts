import {
  mainnet,
  sepolia,
  base,
  baseSepolia,
  optimism,
  optimismSepolia,
  polygon,
  polygonAmoy,
  arbitrum,
  arbitrumNova,
  arbitrumSepolia,
  zksync,
  zksyncSepoliaTestnet,
} from "viem/chains";
import {
  createPublicClient,
  createWalletClient,
  http,
  type Address,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";

import emailDomainProver from "../out/EmailDomainProver.sol/EmailDomainProver";
import emailDomainVerifier from "../out/EmailProofVerifier.sol/EmailDomainVerifier";

import path from "node:path";
import fs from "node:fs/promises";
import dotenv from "dotenv";

const envPath = path.resolve(`${__dirname}/front-app`, ".env.development");
dotenv.config({
  path: envPath,
});

let privateKey = process.env.EXAMPLES_TEST_PRIVATE_KEY;
if (!privateKey) {
  throw new Error("EXAMPLES_TEST_PRIVATE_KEY environment variable is not set.");
}

const jsonRpcUrl = process.env.NEXT_PUBLIC_JSON_RPC_URL;
if (!jsonRpcUrl) {
  throw new Error("NEXT_PUBLIC_JSON_RPC_URL environment variable is not set.");
}

const chainId = Number(process.env.NEXT_PUBLIC_CHAIN_ID);
if (!chainId) {
  throw new Error("NEXT_PUBLIC_CHAIN_ID environment variable is not set.");
}

const supportedChains = {
  [optimismSepolia.id]: optimismSepolia,
  [mainnet.id]: mainnet,
  [sepolia.id]: sepolia,
  [base.id]: base,
  [baseSepolia.id]: baseSepolia,
  [optimism.id]: optimism,
  [polygon.id]: polygon,
  [polygonAmoy.id]: polygonAmoy,
  [arbitrum.id]: arbitrum,
  [arbitrumNova.id]: arbitrumNova,
  [arbitrumSepolia.id]: arbitrumSepolia,
  [zksync.id]: zksync,
  [zksyncSepoliaTestnet.id]: zksyncSepoliaTestnet,
};

const chain = supportedChains[chainId as keyof typeof supportedChains];

privateKey = privateKey.startsWith("0x") ? privateKey : `0x${privateKey}`;
const deployer = privateKeyToAccount(privateKey as `0x${string}`);

const walletClient = createWalletClient({
  chain,
  transport: http(jsonRpcUrl),
});

const client = createPublicClient({
  chain,
  transport: http(jsonRpcUrl),
});

const deployProver = async () => {
  const txHash = await walletClient.deployContract({
    abi: emailDomainProver.abi,
    bytecode: emailDomainProver.bytecode.object,
    account: deployer,
    args: ["@vlayer.xyz"],
    chain,
  });

  const receipt = await client.waitForTransactionReceipt({ hash: txHash });

  if (receipt.status != "success") {
    throw new Error(`Prover deployment failed with status: ${receipt.status}`);
  }

  return receipt.contractAddress as Address;
};

const deployVerifier = async (prover: Address) => {
  const txHash = await walletClient.deployContract({
    abi: emailDomainVerifier.abi,
    bytecode: emailDomainVerifier.bytecode.object,
    account: deployer,
    args: [prover, "vlayer badge", "VL"],
    chain,
  });

  const receipt = await client.waitForTransactionReceipt({ hash: txHash });

  if (receipt.status != "success") {
    throw new Error(
      `Verifier deployment failed with status: ${receipt.status}`,
    );
  }

  return receipt.contractAddress as Address;
};

console.log("Deploying Prover...");
const proverAddr = await deployProver();
console.log(`Prover deployed: ${proverAddr}`);

console.log("Deploying Verifier...");
const verifierAddr = await deployVerifier(proverAddr);
console.log(`Verifier deployed: ${verifierAddr}`);

try {
  await fs.appendFile(envPath, "");
  const envFile = Bun.file(envPath);
  let envContent = await envFile.text();

  if (!envContent) {
    envContent = "";
  }

  const newEnvs = dotenv.parse(envContent);
  newEnvs["NEXT_PUBLIC_PROVER_ADDR"] = proverAddr;
  newEnvs["NEXT_PUBLIC_VERIFIER_ADDR"] = verifierAddr;

  const envLines = Object.entries(newEnvs)
    .map(([key, value]) => `${key}=${value}`)
    .join("\n");
  await Bun.write(envPath, envLines);

  console.log("Successfully updated the front-app/.env.development file");
} catch (err) {
  console.error("Error updating the front-app/.env.development file:", err);
}
