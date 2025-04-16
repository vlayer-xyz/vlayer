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
    if (wagmiChain === configChain) {
      setChain(wagmiChain);
      setError(undefined);
    } else {
      setChain(undefined);
      setError(`Chains mismatched. Wallet chain: ${wagmiChain} is not equal to env chain: ${configChain}`);
    }
  }, [wagmiChain, configChain]);

  return { chain, error };
};
