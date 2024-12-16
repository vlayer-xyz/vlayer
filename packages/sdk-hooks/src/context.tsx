/* eslint-disable @typescript-eslint/no-unsafe-member-access */
import { createVlayerClient } from "@vlayer/sdk";
import { type VlayerContextType } from "./types";
import { createContext, type PropsWithChildren } from "react";
import {
  createContext as createVlayerChainContext,
  customTransport,
} from "@vlayer/sdk/config";
import { createExtensionWebProofProvider } from "@vlayer/sdk/web_proof";
import "viem/window";
import { anvil } from "viem/chains";
import { type Config } from "@vlayer/sdk/config";

export const VlayerContext = createContext<VlayerContextType | null>(null);

export const VlayerProvider = ({
  config,
  children,
}: PropsWithChildren<{
  config: Config & {
    notaryUrl?: string;
    wsProxyUrl?: string;
  };
}>) => {
  const useWindowEthereumTransport = config.chainName !== anvil.name;
  const webProofProvider = createExtensionWebProofProvider({
    notaryUrl: config.notaryUrl,
    wsProxyUrl: config.wsProxyUrl,
  });

  const chainContext = createVlayerChainContext(
    {
      chainName: config.chainName,
      proverUrl: config.proverUrl,
      jsonRpcUrl: config.jsonRpcUrl,
      privateKey: config.privateKey,
    },
    useWindowEthereumTransport && window.ethereum
      ? customTransport(window.ethereum)
      : undefined,
  );

  const vlayerClient = createVlayerClient({
    url: config.proverUrl,
    webProofProvider,
  });

  return (
    <VlayerContext.Provider
      value={{
        vlayerClient,
        webProofProvider,
        chainContext,
      }}
    >
      {children}
    </VlayerContext.Provider>
  );
};
