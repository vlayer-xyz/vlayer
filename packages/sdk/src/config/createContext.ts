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

const isChain = (chain: unknown): chain is Chain => {
  return (
    typeof chain === "object" &&
    chain !== null &&
    "id" in chain &&
    "name" in chain &&
    "nativeCurrency" in chain &&
    "rpcUrls" in chain
  );
};

export const getChainSpecs = (chainName: string): Chain => {
  let chain = undefined;
  try {
    chain = chains[chainName as keyof typeof chains];
  } catch {
    throw Error(`Cannot import ${chainName} from viem/chains`);
  }

  if (!chain || !isChain(chain)) {
    throw new Error(`Chain ${chainName} is not supported by viem`);
  }
  return chain;
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
