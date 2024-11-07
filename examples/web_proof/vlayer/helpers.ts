import {
  createWalletClient,
  publicActions,
  http,
  type Chain,
  type PublicClient,
  type Address,
} from "viem";
import Bun from "bun";
import fs from "node:fs/promises";
import dotenv from "dotenv";

export const getEthClient = (chain: Chain, jsonRpcUrl: string) =>
  createWalletClient({
    chain,
    transport: http(jsonRpcUrl),
  }).extend(publicActions);

export const getContractAddr = async (
  client: PublicClient,
  hash: `0x${string}`,
): Promise<Address> => {
  // 1 confirm at devnet, at least 6 for others
  const confirmations = client?.chain?.name === "Anvil" ? 1 : 6;

  const receipt = await client.waitForTransactionReceipt({
    hash,
    confirmations,
    retryCount: 12,
  });

  if (!receipt.contractAddress || receipt.status != "success")
    throw new Error(
      `Cannot get contract address from receipt: ${receipt.status}`,
    );

  return receipt.contractAddress;
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
