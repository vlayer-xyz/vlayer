import { getChainSpecs } from "@vlayer/sdk/config";
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
  return chain.name.toLowerCase();
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
    if (!configChain) {
      setChain(null);
      setError(`Env chain ${configChain} not found`);
      return;
    }

    try {
      const chain = getChainSpecs(configChain);

      if (!chain) {
        setChain(null);
        setError(`Chain ${configChain} is not supported`);
        return;
      }

      if (wagmiChain === configChain) {
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
  }, [wagmiChain, configChain]);

  return { chain, error };
};
