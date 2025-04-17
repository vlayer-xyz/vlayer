import { getChainSpecs } from "@vlayer/sdk/config";
import { useEffect, useState } from "react";
import { useChainId, useChains } from "wagmi";

export const useChain = () => {
  const [chain, setChain] = useState<string | undefined>(undefined);
  const [error, setError] = useState<string | undefined>(undefined);

  const wagmiChainId = useChainId();
  const wagmiChains = useChains();

  const wagmiChain = wagmiChains
    .find((chain) => chain.id === wagmiChainId)
    ?.name.toLowerCase();

  const configChain = import.meta.env.VITE_VLAYER_CHAIN_ID;

  useEffect(() => {
    if (!configChain) {
      setChain(undefined);
      setError(`Env chain ${configChain} not found`);
      return;
    }

    try {
      const chain = getChainSpecs(configChain);

      if (!chain) {
        setChain(undefined);
        setError(`Chain ${configChain} is not suported`);
        return;
      }

      if (wagmiChain === configChain) {
        setChain(wagmiChain);
        setError(undefined);
      } else {
        setChain(undefined);
        setError(
          `Chains mismatched. Wallet chain: ${wagmiChain} is not equal to env chain: ${configChain}`,
        );
      }
    } catch {
      setChain(undefined);
      setError(`Chain ${configChain} is not suported`);
    }
  }, [wagmiChain, configChain]);

  return { chain, error };
};
