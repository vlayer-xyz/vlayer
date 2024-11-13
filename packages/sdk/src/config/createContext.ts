import { Chain, createWalletClient, http, publicActions } from "viem";
import { Config } from "./getConfig";
import { privateKeyToAccount } from "viem/accounts";
import { getChainConfirmations } from "./getChainConfirmations";

const getChainSpecs = async (chainName: string): Promise<Chain> => {
  try {
    const chains = await import("viem/chains");
    const chain = chains[chainName as keyof typeof chains] as Chain;
    return chain;
  } catch {
    throw Error(`Cannot import ${chainName} from viem/chains`);
  }
};

const createEthClient = (chain: Chain, jsonRpcUrl: string) =>
  createWalletClient({
    chain,
    transport: http(jsonRpcUrl),
  }).extend(publicActions);

export const createContext = async (config: Config) => {
  const chain = await getChainSpecs(config.chainName);
  const jsonRpcUrl = config.jsonRpcUrl ?? chain.rpcUrls.default.http[0];

  return {
    ...config,
    chain,
    deployer: privateKeyToAccount(config.privateKey),
    jsonRpcUrl,
    ethClient: createEthClient(chain, jsonRpcUrl),
    confirmations: getChainConfirmations(config.chainName),
  };
};
