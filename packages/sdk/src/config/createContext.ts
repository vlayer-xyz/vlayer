import {
  type Chain,
  createWalletClient,
  http,
  publicActions,
  type CustomTransport,
  custom,
} from "viem";
import { Config } from "./getConfig";
import { privateKeyToAccount } from "viem/accounts";
import { getChainConfirmations } from "./getChainConfirmations";

const getChainSpecs = async (chainName: string): Promise<Chain> => {
  try {
    const chains = await import("viem/chains");
    return chains[chainName as keyof typeof chains] as Chain;
  } catch {
    throw Error(`Cannot import ${chainName} from viem/chains`);
  }
};

export const customTransport = custom;
export { Chain };
const createEthClient = (
  chain: Chain,
  jsonRpcUrl: string,
  transport?: CustomTransport,
) =>
  createWalletClient({
    chain,
    transport: transport || http(jsonRpcUrl),
  }).extend(publicActions);

export const createContext = async (
  config: Config,
  transport?: CustomTransport,
) => {
  const chain = await getChainSpecs(config.chainName);
  const jsonRpcUrl = config.jsonRpcUrl ?? chain.rpcUrls.default.http[0];

  return {
    ...config,
    chain,
    account: privateKeyToAccount(config.privateKey),
    jsonRpcUrl,
    ethClient: createEthClient(chain, jsonRpcUrl, transport),
    confirmations: getChainConfirmations(config.chainName),
  };
};
