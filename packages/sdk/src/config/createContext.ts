import {
  type Chain,
  createWalletClient,
  http,
  publicActions,
  type CustomTransport,
  custom,
} from "viem";
import { type Config } from "./getConfig";
import { privateKeyToAccount } from "viem/accounts";
import { getChainConfirmations } from "./getChainConfirmations";
import * as chains from "viem/chains";

const getChainSpecs = (chainName: string): Chain => {
  try {
    return chains[chainName as keyof typeof chains] as Chain;
  } catch {
    throw Error(`Cannot import ${chainName} from viem/chains`);
  }
};

export const customTransport = custom;
export type { Chain };
const createEthClient = (
  chain: Chain,
  jsonRpcUrl: string,
  transport?: CustomTransport,
) =>
  createWalletClient({
    chain,
    transport: transport || http(jsonRpcUrl),
  }).extend(publicActions);

export const createContext = (config: Config, transport?: CustomTransport) => {
  const chain = getChainSpecs(config.chainName);
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
