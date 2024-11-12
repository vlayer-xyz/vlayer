import {
  createWalletClient,
  publicActions,
  http,
  type Chain,
  type PublicClient,
  type Address,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";

import { type Config } from "./config";
import Bun from "bun";
import fs from "node:fs/promises";
import dotenv from "dotenv";
import debug from "debug";
const log = debug("vlayer:config");

const importChainSpecs = async (chainName: string): Promise<Chain> => {
  try {
    const chains = await import(`viem/chains`);
    const chain = chains[chainName as keyof typeof chains] as Chain;
    return chain;
  } catch {
    throw Error(`Cannot import ${chainName} from viem/chains`);
  }
};

export const getEthClient = (chain: Chain, jsonRpcUrl: string) =>
  createWalletClient({
    chain,
    transport: http(jsonRpcUrl),
  }).extend(publicActions);

const chainConfirmations = (chainName?: string): number => {
  if (!chainName || chainName.toLowerCase() === "anvil") {
    return 1;
  } else {
    return 6;
  }
};

export const waitForContractAddr = async (
  client: PublicClient,
  hash: `0x${string}`,
): Promise<Address> => {
  const receipt = await client.waitForTransactionReceipt({
    hash,
    confirmations: chainConfirmations(client?.chain?.name),
    retryCount: 120,
    retryDelay: 1000,
  });

  if (!receipt.contractAddress || receipt.status != "success")
    throw new Error(
      `Cannot get contract address from receipt: ${receipt.status}`,
    );

  return receipt.contractAddress;
};

export const exampleContext = async (config: Config) => {
  const chain = await importChainSpecs(config.chainName);
  const jsonRpcUrl = config.jsonRpcUrl ?? chain.rpcUrls.default.http[0];

  return {
    ...Object.assign(config, {
      chain,
      deployer: privateKeyToAccount(config.privateKey),
      jsonRpcUrl,
      ethClient: await getEthClient(chain, jsonRpcUrl),
      confirmations: chainConfirmations(config.chainName),
    }),
  };
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

  log(`Successfully updated the ${envPath} with: `, overrides);
};
