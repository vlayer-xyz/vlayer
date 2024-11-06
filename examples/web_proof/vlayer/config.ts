import path from "node:path";
import { privateKeyToAccount } from "viem/accounts";
import { type PrivateKeyAccount, type Chain } from "viem";
import { anvil } from "viem/chains";
import dotenv from "dotenv";

interface Config {
  chainName: string;
  chain: Chain;
  proverUrl: string;
  privateKey: string;
  deployer: PrivateKeyAccount | null;
  jsonRpcUrl: string;
  envPath: string;
}

const DEFAULT_CONFIG: Config = {
  chainName: "anvil",
  chain: anvil,
  proverUrl: "http://127.0.0.1:3000",
  privateKey:
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", // default anvil key
  deployer: null,
  jsonRpcUrl: "",
  envPath: "",
};

export const getConfig = async (envPath?: string) => {
  if (!envPath) envPath = path.resolve(__dirname, ".env.development");
  dotenv.config({ path: envPath });

  const chainName = process.env.CHAIN_NAME ?? DEFAULT_CONFIG.chainName;
  const privateKey =
    (process.env.EXAMPLES_TEST_PRIVATE_KEY as `0x${string}`) ??
    DEFAULT_CONFIG.privateKey;
  const chains = await import(`viem/chains`);
  const chain = chains[chainName as keyof typeof chains] as Chain;
  const jsonRpcUrl = process.env.JSON_RPC_URL ?? chain.rpcUrls.default.http[0];

  return {
    ...Object.assign(DEFAULT_CONFIG, {
      chainName,
      privateKey,
      deployer: privateKeyToAccount(privateKey),
      jsonRpcUrl,
      envPath,
    }),
  };
};
