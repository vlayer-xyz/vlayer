import path from "node:path";
import dotenv from "dotenv";

export type Config = {
  chainName: string;
  proverUrl: string;
  jsonRpcUrl: string;
  envPath: string;
  privateKey: `0x${string}`;
};

const DEFAULT_CONFIG: Config = {
  chainName: "anvil",
  proverUrl: "http://127.0.0.1:3000",
  jsonRpcUrl: "",
  envPath: "",
  // anvil default private key
  privateKey:
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
};

export const getConfig = (envPath?: string): Config => {
  const dotEnvFileName = `.env.${process.env.VLAYER_ENV ?? "development"}`;
  if (!envPath) envPath = path.resolve(__dirname, dotEnvFileName);
  dotenv.config({ path: envPath, override: true });
  dotenv.config({ path: `${envPath}.local`, override: true });

  const chainName = process.env.CHAIN_NAME || DEFAULT_CONFIG.chainName;

  const privateKey =
    (process.env.EXAMPLES_TEST_PRIVATE_KEY as `0x${string}`) ||
    DEFAULT_CONFIG.privateKey;

  return {
    ...Object.assign(DEFAULT_CONFIG, {
      chainName,
      privateKey,
      jsonRpcUrl: process.env.JSON_RPC_URL,
      envPath,
    }),
  };
};
