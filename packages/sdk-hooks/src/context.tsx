import { createVlayerClient } from "@vlayer/sdk";
import { type VlayerContextType } from "./types";
import { createContext, useCallback, type PropsWithChildren } from "react";
import { createExtensionWebProofProvider } from "@vlayer/sdk/web_proof";
import "viem/window";
import { ErrorBoundary } from "react-error-boundary";
import { WagmiProviderNotFoundError } from "wagmi";
export const VlayerContext = createContext<VlayerContextType | null>(null);

const NO_WAGMI_PROVIDER_ERROR_MESSAGE =
  "Wagmi provider is required but not found. Please make sure you have connected a Wagmi provider.";

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

  const handleError = useCallback((error: Error) => {
    if (error instanceof WagmiProviderNotFoundError) {
      console.error(`@vlayer/react: ${NO_WAGMI_PROVIDER_ERROR_MESSAGE}`);
    }
  }, []);
  return (
    <ErrorBoundary
      FallbackComponent={VlayerErrorFallback}
      onError={handleError}
    >
      <VlayerContext.Provider
        value={{
          vlayerClient,
          webProofProvider,
        }}
      >
        {children}
      </VlayerContext.Provider>
    </ErrorBoundary>
  );
};

function VlayerErrorFallback({ error }: { error: Error }) {
  if (error instanceof WagmiProviderNotFoundError) {
    return (
      <div style={{ textAlign: "center", fontSize: "1.2rem" }}>
        {NO_WAGMI_PROVIDER_ERROR_MESSAGE}
      </div>
    );
  }

  return <div>Vlayer Error: {error.message}</div>;
}
