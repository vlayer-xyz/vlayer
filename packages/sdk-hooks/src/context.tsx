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
import { type Config } from "@vlayer/sdk/config";

export const VlayerContext = createContext<VlayerContextType | null>(null);

export const VlayerProvider = ({
  config,
  children,
}: PropsWithChildren<{
  config: Config & { useWindowEthereumTransport: boolean };
}>) => {
  return (
    <VlayerContext.Provider
      value={{
        vlayerClient: createVlayerClient({
          url: config.proverUrl,
        }),
        webProofProvider: createExtensionWebProofProvider(),
        chainContext: createVlayerChainContext(
          {
            chainName: config.chainName,
            proverUrl: config.proverUrl,
            jsonRpcUrl: config.jsonRpcUrl,
            privateKey: config.privateKey,
          },
          config.useWindowEthereumTransport && window.ethereum
            ? customTransport(window.ethereum)
            : undefined,
        ),
      }}
    >
      {children}
    </VlayerContext.Provider>
  );
};
