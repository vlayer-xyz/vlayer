import { getChainSpecs } from "@vlayer/sdk";
import { useEffect, useMemo, useState } from "react";
import type { Chain } from "viem";
import { useAccount, useChains, useSwitchChain } from "wagmi";

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

export class ChainSwitchError extends Error {
  constructor(chainName: string) {
    super(
      `Failed to switch to ${chainName} make sure you have it in your wallet`,
    );
    this.name = "ChainSwitchError";
  }
}

export class ChainNotSupportedError extends Error {
  constructor(chainName: string) {
    super(`Chain ${chainName} is not supported`);
    this.name = "ChainNotSupportedError";
  }
}

export class MissingChainError extends Error {
  constructor() {
    super("Env chain not defined");
    this.name = "MissingChainError";
  }
}

/**
 * @description This hook is used to make sure the chain in the environment variable is the same as the chain in the wallet.
 * @param configChain - The chain name in the environment variable.
 * @returns The chain object and the error object.
 * @throws {MissingChainError} - If the chain name in the environment variable is not defined.
 * @throws {ChainNotSupportedError} - If the chain name in the environment variable is not supported by viem.
 * @throws {ChainSwitchError} - If the chain name provided by the environment variable is not the same as the chain in the wallet
 * and the switch fails (most likely because the wallet does not have the chain)
 */

export const useSyncChain = (
  configChain: string | undefined,
): {
  chain: Chain | null;
  error: MissingChainError | ChainNotSupportedError | ChainSwitchError | null;
  switched: boolean;
} => {
  const { switchChain } = useSwitchChain();
  const [chain, setChain] = useState<Chain | null>(null);
  const [error, setError] = useState<Error | null>(null);
  const [switched, setSwitched] = useState<boolean>(false);
  const { chainId: wagmiChainId } = useAccount();
  const wagmiChains = useChains();

  const wagmiChain = useMemo(() => {
    return wagmiChainId ? findChainById(wagmiChains, wagmiChainId) : undefined;
  }, [wagmiChainId, wagmiChains]);

  useEffect(() => {
    if (configChain === undefined) {
      setChain(null);
      setError(new MissingChainError());
      return;
    }

    try {
      const chain = getChainSpecs(configChain);
      if (!chain) {
        setChain(null);
        setError(new ChainNotSupportedError(configChain));
        return;
      }

      if (wagmiChainId === chain.id) {
        setChain(chain);
        setError(null);
      } else {
        switchChain(
          { chainId: chain.id },
          {
            onError: (e) => {
              console.error("chain switch error", chain.name, chain.id, e);
              setError(new ChainSwitchError(chain.name));
            },
            onSuccess: () => {
              setSwitched(true);
            },
          },
        );
      }
    } catch {
      setChain(null);
      setError(new ChainNotSupportedError(configChain));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [wagmiChainId, configChain, wagmiChain]);

  return { chain, error, switched };
};
