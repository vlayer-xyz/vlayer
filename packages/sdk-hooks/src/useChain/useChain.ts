import { useChainId, useChains } from "wagmi";

export const useChain = () => {
  let error: string | undefined = undefined;
  let chain: string | undefined = undefined;

  const wagmiChainId = useChainId();
  const wagmiChains = useChains();

  const wagmiChain = wagmiChains
    .find((chain) => chain.id === wagmiChainId)
    ?.name.toLowerCase();

  const configChain = import.meta.env.VITE_VLAYER_CHAIN_ID;

  if (wagmiChain === configChain) {
    chain = wagmiChain;
  } else {
    error = `Chains mismatched. Wallet chain: ${wagmiChain} is not equal to env chain: ${configChain}`;
  }

  return { chain, error };
};
