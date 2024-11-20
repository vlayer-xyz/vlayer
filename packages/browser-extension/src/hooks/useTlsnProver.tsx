import { prove as tlsnProve } from "tlsn-js";
import React, {
  createContext,
  PropsWithChildren,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";
import { formatTlsnHeaders } from "lib/formatTlsnHeaders";
import { isDefined, ExtensionMessageType } from "../web-proof-commons";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { useProvenUrl } from "./useProvenUrl";
import { useTrackHistory } from "hooks/useTrackHistory";
import { removeQueryParams } from "lib/removeQueryParams";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";

const TlsnProofContext = createContext({
  prove: async () => {},
  proof: null as object | null,
  isProving: false,
});
export const TlsnProofContextProvider = ({ children }: PropsWithChildren) => {
  const [proof, setProof] = useState<object | null>(null);
  const [isProving, setIsProving] = useState(false);
  const [formattedHeaders, setFormattedHeaders] = useState<{
    headers: Record<string, string>;
    secretHeaders: string[];
  }>({
    headers: {},
    secretHeaders: [],
  });

  useTrackHistory();
  const [provingSessionConfig] = useProvingSessionConfig();
  const provenUrl = useProvenUrl();

  useEffect(() => {
    setFormattedHeaders(
      formatTlsnHeaders(provenUrl?.headers ?? [], provenUrl?.cookies ?? []),
    );
  }, [provenUrl]);

  const prove = useCallback(async () => {
    setIsProving(true);

    const progressInterval = setInterval(() => {
      void sendMessageToServiceWorker({
        type: ExtensionMessageType.ProofProcessing,
        payload: {},
      });
    }, 1000);

    try {
      isDefined(provenUrl?.url, "Missing URL to prove");
      isDefined(provingSessionConfig, "Missing proving session config");

      const tlsnProof = await tlsnProve(removeQueryParams(provenUrl.url), {
        notaryUrl: provingSessionConfig.notaryUrl || "",
        websocketProxyUrl: `${provingSessionConfig.wsProxyUrl}?token=${new URL(provenUrl.url).host}`,
        method: "GET",
        headers: formattedHeaders?.headers,
        secretHeaders: formattedHeaders?.secretHeaders,
      });

      void sendMessageToServiceWorker({
        type: ExtensionMessageType.ProofDone,
        payload: {
          proof: tlsnProof,
        },
      });

      setProof(tlsnProof);
    } catch (e: unknown) {
      void sendMessageToServiceWorker({
        type: ExtensionMessageType.ProofError,
        payload: {
          error: e instanceof Error ? e.message : String(e),
        },
      });
    } finally {
      setIsProving(false);
      clearInterval(progressInterval);
    }
  }, [provenUrl, formattedHeaders, provingSessionConfig]);

  return (
    <TlsnProofContext.Provider value={{ prove, proof, isProving }}>
      {children}
    </TlsnProofContext.Provider>
  );
};

export const useTlsnProver = () => {
  return useContext(TlsnProofContext);
};
