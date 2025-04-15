import { createVlayerClient } from "@vlayer/sdk";
import { createContext, useContext, type PropsWithChildren } from "react";
import { createExtensionWebProofProvider } from "@vlayer/sdk/web_proof";
import {
  type ProofContextType,
  type ProofConfig,
  type WebProofContextType,
  type ProverContextType,
} from "./types";
import { DEFAULT_CONFIG, DEFAULT_CONFIG_ENV } from "./defaults";

export const ProofContext = createContext<ProofContextType | null>(null);
export const WebProofContext = createContext<WebProofContextType | null>(null);
export const ProverContext = createContext<ProverContextType | null>(null);

export const ProofProvider = ({
  config,
  children,
}: PropsWithChildren<{
  config?: Partial<ProofConfig>;
}>) => {
  const { proverUrl, notaryUrl, wsProxyUrl, token } = {
    ...DEFAULT_CONFIG[config?.env ?? DEFAULT_CONFIG_ENV],
    ...config,
  };

  const webProofProvider = createExtensionWebProofProvider({
    notaryUrl: notaryUrl,
    wsProxyUrl: wsProxyUrl,
    token,
  });

  const vlayerClient = createVlayerClient({
    url: proverUrl,
    webProofProvider,
    token,
  });

  return (
    <WebProofContext.Provider
      value={{ webProofProvider, config: { notaryUrl, wsProxyUrl } }}
    >
      <ProverContext.Provider value={{ vlayerClient, config: { proverUrl } }}>
        {children}
      </ProverContext.Provider>
    </WebProofContext.Provider>
  );
};

export const useProofContext = () => {
  const webProofContext = useContext(WebProofContext);
  const proverContext = useContext(ProverContext);

  if (!webProofContext || !proverContext) {
    throw new Error("useProofContext must be used within a ProofProvider");
  }
  return {
    ...webProofContext,
    ...proverContext,
    config: { ...webProofContext.config, ...proverContext.config },
  };
};

export const useWebProofContext = () => {
  const webProofContext = useContext(WebProofContext);
  if (!webProofContext) {
    throw new Error(
      "useWebProofContext must be used within a WebProofProvider",
    );
  }
  return webProofContext;
};

export const useProverContext = () => {
  const proverContext = useContext(ProverContext);
  if (!proverContext) {
    throw new Error("useProverContext must be used within a ProverProvider");
  }
  return proverContext;
};
