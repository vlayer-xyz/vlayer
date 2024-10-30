import {
  createPublicClient,
  createWalletClient,
  http,
  type Address,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { optimismSepolia } from "viem/chains";

import emailDomainProver from "../out/EmailDomainProver.sol/EmailDomainProver";
import emailDomainVerifier from "../out/EmailProofVerifier.sol/EmailDomainVerifier";

import path from "node:path";
import fs from "node:fs/promises";
import dotenv from "dotenv";

const envPath = path.resolve(`${__dirname}/front-app`, ".env.local");
dotenv.config({ path: envPath });

const getConfig = () => {
  const config = {
    deployer: privateKeyToAccount(
      process.env.EXAMPLES_TEST_PRIVATE_KEY as `0x${string}`,
    ),
    domain: "@vlayer.xyz",
    chain: optimismSepolia,
    jsonRpcUrl: process.env.VITE_JSON_RPC_URL,
  };
  if (!config.jsonRpcUrl)
    config.jsonRpcUrl = config.chain.rpcUrls.default.http[0];

  return config;
};

const config = getConfig();

const walletClient = createWalletClient({
  chain: config.chain,
  transport: http(config.jsonRpcUrl),
});

const client = createPublicClient({
  chain: config.chain,
  transport: http(config.jsonRpcUrl),
});

const deployProver = async () => {
  const txHash = await walletClient.deployContract({
    abi: emailDomainProver.abi,
    bytecode: emailDomainProver.bytecode.object,
    account: config.deployer,
    args: [config.domain],
    chain: config.chain,
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
    account: config.deployer,
    args: [prover, "vlayer badge", "VL"],
    chain: config.chain,
  });

  const receipt = await client.waitForTransactionReceipt({ hash: txHash });

  if (receipt.status != "success") {
    throw new Error(
      `Verifier deployment failed with status: ${receipt.status}`,
    );
  }

  return receipt.contractAddress as Address;
};

const updateDotFile = async (
  envPath: string,
  overrides: { [key: string]: string },
) => {
  try {
    await fs.appendFile(envPath, "");
    const envFile = Bun.file(envPath);
    let envContent = await envFile.text();

    if (!envContent) {
      envContent = "";
    }

    const newEnvs = Object.assign(dotenv.parse(envContent), overrides);

    const envLines = Object.entries(newEnvs)
      .map(([key, value]) => `${key}=${value}`)
      .join("\n");
    await Bun.write(envPath, envLines);

    console.log(
      `Successfully updated the front-app/.env.development with ${Object.keys(overrides)[0]}`,
    );
  } catch (err) {
    console.error(
      `Error updating the front-app/.env.development ${Object.keys(overrides)[0]}`,
      err,
    );
  }
};

console.log("Deploying Prover...");
const proverAddr = await deployProver();
console.log(`Prover deployed: ${proverAddr}`);
await updateDotFile(envPath, { VITE_PROVER_ADDR: proverAddr });

console.log("Deploying Verifier...");
const verifierAddr = await deployVerifier(proverAddr);
console.log(`Verifier deployed: ${verifierAddr}`);
await updateDotFile(envPath, { VITE_VERIFIER_ADDR: verifierAddr });
