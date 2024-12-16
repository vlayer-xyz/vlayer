import { createVlayerClient } from "@vlayer/sdk";
import { type VlayerContextType } from "./types";
import { createContext, type PropsWithChildren } from "react";
import { createExtensionWebProofProvider } from "@vlayer/sdk/web_proof";
import "viem/window";
export const VlayerContext = createContext<VlayerContextType | null>(null);

const DEFAULT_PROVER_URL = "https://test-prover.vlayer.xyz";
const DEFAULT_NOTARY_URL = "https://test-notary.vlayer.xyz";
const DEFAULT_WS_PROXY_URL = "wss://test-wsproxy.vlayer.xyz";

const DEFAULT_CONFIG = {
  proverUrl: DEFAULT_PROVER_URL,
  notaryUrl: DEFAULT_NOTARY_URL,
  wsProxyUrl: DEFAULT_WS_PROXY_URL,
};

export const VlayerProvider = ({
  config,
  children,
}: PropsWithChildren<{
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
      }}
    >
      {children}
    </VlayerContext.Provider>
  );
};
