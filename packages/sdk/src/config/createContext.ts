import {
  type Chain,
  createWalletClient,
  http,
  publicActions,
  type CustomTransport,
  custom,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { getChainConfirmations } from "./utils/getChainConfirmations";
import * as chains from "viem/chains";
import type { VlayerContextConfig } from "./types";

export type EthClient = ReturnType<typeof createContext>["ethClient"];

const getChainSpecs = (chainName: string): Chain => {
  try {
    return chains[chainName as keyof typeof chains] as Chain;
  } catch {
    throw Error(`Cannot import ${chainName} from viem/chains`);
  }
};

export const customTransport = custom;

const createEthClient = (
  chain: Chain,
  jsonRpcUrl: string,
  transport?: CustomTransport,
) =>
  createWalletClient({
    chain,
    transport: transport || http(jsonRpcUrl),
  }).extend(publicActions);

export function createContext(
  config: VlayerContextConfig,
  transport?: CustomTransport,
) {
  const chain = getChainSpecs(config.chainName);
  const jsonRpcUrl = config.jsonRpcUrl ?? chain.rpcUrls.default.http[0];

  return {
    ...config,
    chain,
    account: config.privateKey && privateKeyToAccount(config.privateKey),
    jsonRpcUrl,
    ethClient: createEthClient(chain, jsonRpcUrl, transport),
    confirmations: getChainConfirmations(config.chainName),
  };
}
