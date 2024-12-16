import React from "react";
import { createVlayerClient } from "@vlayer/sdk";
import { createContext, type PropsWithChildren } from "react";
import { createExtensionWebProofProvider } from "@vlayer/sdk/web_proof";
<<<<<<< HEAD
import { type VlayerContextType } from "./types";
=======
import "viem/window";
import { anvil } from "viem/chains";
import { type Config } from "@vlayer/sdk/config";

>>>>>>> 916b4880 (Add example usage of provider)
export const VlayerContext = createContext<VlayerContextType | null>(null);

const DEFAULT_CONFIG = {
  proverUrl: "https://test-prover.vlayer.xyz",
  notaryUrl: "https://test-notary.vlayer.xyz",
  wsProxyUrl: "wss://test-wsproxy.vlayer.xyz",
};

export const VlayerProvider = ({
  config,
  children,
}: PropsWithChildren<{
<<<<<<< HEAD
  config: Partial<typeof DEFAULT_CONFIG>;
}>) => {
  const { proverUrl, notaryUrl, wsProxyUrl } = { ...DEFAULT_CONFIG, ...config };

  const webProofProvider = createExtensionWebProofProvider({
    notaryUrl: notaryUrl,
    wsProxyUrl: wsProxyUrl,
  });

  const vlayerClient = createVlayerClient({
    url: proverUrl,
    webProofProvider,
  });

  return (
    <VlayerContext.Provider
      value={{
        vlayerClient,
        webProofProvider,
=======
  config: Config;
}>) => {
  const useWindowEthereumTransport = config.chainName !== anvil.name;
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
          useWindowEthereumTransport && window.ethereum
            ? customTransport(window.ethereum)
            : undefined,
        ),
>>>>>>> 916b4880 (Add example usage of provider)
      }}
    >
      {children}
    </VlayerContext.Provider>
  );
};
