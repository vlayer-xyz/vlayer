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
import type { VlayerContextConfig } from "./types";
import { getChainSpecs } from "../utils";

export type EthClient = ReturnType<typeof createContext>["ethClient"];

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
): {
  chain: Chain;
  account?: ReturnType<typeof privateKeyToAccount>;
  jsonRpcUrl: string;
  ethClient: ReturnType<typeof createEthClient>;
  confirmations: number;
} & VlayerContextConfig {
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
