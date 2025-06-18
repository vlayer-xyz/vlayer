import { getChainSpecs } from "@vlayer/sdk";
import { useEffect, useReducer } from "react";
import type { Chain } from "viem";
import { useAccount, useSwitchChain } from "wagmi";
import { reducer } from "./reducer";
import {
  ChainSwitchError,
  ChainNotSupportedError,
  MissingConfigChainError,
} from "./errors";
import { debug } from "debug";

const log = debug("vlayer:sdk-hooks:useSyncChain");

/**
 * @description This hook is used to make sure the chain in the environment variable is the same as the chain in the wallet.
 * @param configChain - The chain name in the environment variable.
 * @returns The chain object and the error object.
 * @returns {MissingChainError} - If the chain name in the environment variable is not defined.
 * @returns {ChainNotSupportedError} - If the chain name in the environment variable is not supported by viem.
 * @returns {ChainSwitchError} - If the chain name provided by the environment variable is not the same as the chain in the wallet
 * and the switch fails (most likely because the wallet does not have the chain)
 */

export const useSyncChain = (
  configChain: string | undefined,
): {
  chain: Chain | null;
  error:
    | MissingConfigChainError
    | ChainNotSupportedError
    | ChainSwitchError
    | null;
  switched: boolean;
} => {
  const { switchChain } = useSwitchChain();

  const [state, dispatch] = useReducer(reducer, {
    chain: null,
    error: null,
    switched: false,
  });
  const { chainId: wagmiChainId } = useAccount();

  useEffect(() => {
    if (configChain === undefined) {
      dispatch({ type: "NO_CHAIN" });
      return;
    }

    let chain;

    try {
      chain = getChainSpecs(configChain);
    } catch {
      dispatch({ type: "CHAIN_NOT_SUPPORTED", payload: configChain });
      return;
    }

    if (!chain) {
      dispatch({ type: "CHAIN_NOT_SUPPORTED", payload: configChain });
      return;
    }

    if (wagmiChainId === chain.id) {
      dispatch({ type: "CHAIN_IN_SYNC", payload: chain });
    } else {
      switchChain(
        { chainId: chain.id },
        {
          onError: (e) => {
            log("error switching chain", e);
            dispatch({ type: "CHAIN_SWITCH_ERROR", payload: chain.name });
          },
          onSuccess: () => {
            log("success switching chain", chain);
            dispatch({ type: "CHAIN_SWITCHED", payload: chain });
          },
        },
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [wagmiChainId]);

  return state;
};
