import { createVlayerClient } from "@vlayer/sdk";
import React, {
  createContext,
  useContext,
  type PropsWithChildren,
} from "react";
import { createExtensionWebProofProvider } from "@vlayer/sdk/web_proof";
import { type VlayerContextType } from "./types";
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
  config?: Partial<typeof DEFAULT_CONFIG>;
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
      }}
    >
      {children}
    </VlayerContext.Provider>
  );
};

export const useVlayerContext = () => {
  const context = useContext(VlayerContext);
  if (!context) {
    throw new Error("useVlayerContext must be used within a VlayerProvider");
  }
  return context;
};
