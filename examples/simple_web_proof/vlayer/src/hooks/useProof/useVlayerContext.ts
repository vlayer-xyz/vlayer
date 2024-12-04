import { customTransport } from "@vlayer/sdk/config";
import { createContext } from "@vlayer/sdk/config";
import { useMemo } from "react";
import { type EIP1193Provider } from "viem";

declare global {
  interface Window {
    ethereum?: EIP1193Provider;
  }
}

export const useVlayerContext = () => {
  const { ethClient: walletClient } = useMemo(
    () =>
      createContext(
        {
          chainName: import.meta.env.VITE_CHAIN_NAME,
          proverUrl: import.meta.env.VITE_PROVER_URL,
          jsonRpcUrl: import.meta.env.VITE_JSON_RPC_URL,
          privateKey: import.meta.env.VITE_PRIVATE_KEY,
        },
        import.meta.env.VITE_USE_WINDOW_ETHEREUM_TRANSPORT
          ? customTransport(window.ethereum as EIP1193Provider)
          : undefined,
      ),
    [],
  );

  return {
    walletClient,
  };
};
