import type { Chain } from "viem";
import * as chains from "viem/chains";

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
    throw new Error(`Cannot import ${chainName} from viem/chains`);
  }

  if (!chain || !isChain(chain)) {
    throw new Error(`Chain ${chainName} is not supported by viem`);
  }
  return chain;
};
