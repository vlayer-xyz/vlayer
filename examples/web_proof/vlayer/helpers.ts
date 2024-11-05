import { privateKeyToAccount } from "viem/accounts";
import { createPublicClient, createWalletClient, http } from "viem";
import Bun from "bun";
import fs from "node:fs/promises";
import dotenv from "dotenv";

export const loadDotFile = async (envPath: string) => {
  dotenv.config({ path: envPath });
};

export const updateDotFile = async (
  envPath: string,
  overrides: { [key: string]: string },
) => {
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

  console.log(`Successfully updated the ${envPath} with: `, overrides);
};

export const getConfig = async () => {
  const chainName = process.env.CHAIN_NAME ?? "anvil";
  const privateKey = process.env.EXAMPLES_TEST_PRIVATE_KEY as `0x${string}`;
  const { [chainName]: chain } = await import(`viem/chains`);
  const jsonRpcUrl = process.env.JSON_RPC_URL ?? chain.rpcUrls.default.http[0];

  const walletClient = createWalletClient({
    chain,
    transport: http(jsonRpcUrl),
  });

  const publicClient = createPublicClient({
    chain,
    transport: http(jsonRpcUrl),
  });

  return {
    chainName,
    chain,
    proverUrl: process.env.PROVER_URL ?? "http://127.0.0.1:3000",
    privateKey,
    deployer: privateKeyToAccount(privateKey),
    jsonRpcUrl,
    walletClient,
    publicClient,
  };
};
