import {
  type Chain,
  createWalletClient,
  http,
  publicActions,
  type CustomTransport,
  custom,
  type PrivateKeyAccount,
  type Address,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { getChainConfirmations } from "./getChainConfirmations";
import * as chains from "viem/chains";
import type { EnvConfig, VlayerContextConfig } from "./types";

const getChainSpecs = (chainName: string): Chain => {
  try {
    return chains[chainName as keyof typeof chains] as Chain;
  } catch {
    throw Error(`Cannot import ${chainName} from viem/chains`);
  }
};

export const customTransport = custom;
export type { Chain, PrivateKeyAccount, Address };
const createEthClient = (
  chain: Chain,
  jsonRpcUrl: string,
  transport?: CustomTransport,
) =>
  createWalletClient({
    chain,
    transport: transport || http(jsonRpcUrl),
  }).extend(publicActions);

export function createContext(config: EnvConfig): {
  chain: Chain;
  account: ReturnType<typeof privateKeyToAccount>;
  jsonRpcUrl: string;
  ethClient: ReturnType<typeof createEthClient>;
  confirmations: number;
} & EnvConfig;

export function createContext(
  config: VlayerContextConfig,
  transport?: CustomTransport,
): {
  chain: Chain;
  jsonRpcUrl: string;
  account: PrivateKeyAccount;
  ethClient: ReturnType<typeof createEthClient>;
  confirmations: number;
} & VlayerContextConfig;

export function createContext(
  config: VlayerContextConfig | EnvConfig,
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
