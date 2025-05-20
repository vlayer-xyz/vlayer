import { getChainSpecs } from "@vlayer/sdk";
import { useEffect, useMemo, useState } from "react";
import type { Chain } from "viem";
import { useChainId, useChains } from "wagmi";

const findChainById = (
  chains: readonly [Chain, ...Chain[]],
  chainId: number,
) => {
  const chain = chains.find((chain) => chain.id === chainId);

  if (!chain) {
    return undefined;
  }
  return chain.name;
};

export const useChain = (
  configChain: string | undefined,
): { chain: Chain | null; error: string | null } => {
  const [chain, setChain] = useState<Chain | null>(null);
  const [error, setError] = useState<string | null>(null);

  const wagmiChainId = useChainId();
  const wagmiChains = useChains();

  const wagmiChain = useMemo(() => {
    return findChainById(wagmiChains, wagmiChainId);
  }, [wagmiChainId, wagmiChains]);

  useEffect(() => {
    if (configChain === undefined) {
      setChain(null);
      setError(`Env chain not defined`);
      return;
    }

    try {
      const chain = getChainSpecs(configChain);

      if (!chain) {
        setChain(null);
        setError(`Chain ${configChain} is not supported`);
        return;
      }

      if (wagmiChainId === chain.id) {
        setChain(chain);
        setError(null);
      } else {
        setChain(null);
        setError(
          `Chains mismatched. Wallet chain: ${wagmiChain} is not equal to env chain: ${configChain}`,
        );
      }
    } catch {
      setChain(null);
      setError(`Chain ${configChain} is not supported`);
    }
  }, [wagmiChainId, configChain, wagmiChain]);

  return { chain, error };
};
