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
  //Internal component state representing proving mechanism
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

  // format headers to make it accepted by tlsn js api
  useEffect(() => {
    setFormattedHeaders(
      formatTlsnHeaders(provenUrl?.headers ?? [], provenUrl?.cookies ?? []),
    );
  }, [provenUrl?.url, provenUrl?.headers, provenUrl?.cookies]);

  const prove = useCallback(async () => {
    console.log("Proving...", provenUrl);
    setIsProving(true);

    try {
      isDefined(provenUrl?.url, "Missing URL to prove ");
      isDefined(provingSessionConfig, "Missing proving session config");

      console.log("Proving", removeQueryParams(provenUrl?.url));
      console.log("Proving", provingSessionConfig.notaryUrl);
      console.log(
        "Proving",
        `${provingSessionConfig.wsProxyUrl}?token=${new URL(provenUrl.url).host}`,
      );
      console.log("Proving", "GET");
      console.log("Proving", formattedHeaders?.headers);
      console.log("Proving", formattedHeaders?.secretHeaders);

      const tlsnProof = await tlsnProve(removeQueryParams(provenUrl?.url), {
        notaryUrl: provingSessionConfig.notaryUrl || "",
        websocketProxyUrl: `${provingSessionConfig.wsProxyUrl}?token=${new URL(provenUrl.url).host}`,
        method: "GET",
        headers: formattedHeaders?.headers,
        secretHeaders: formattedHeaders?.secretHeaders,
      });
      // let service worker know proof is done
      await sendMessageToServiceWorker({
        type: ExtensionMessageType.ProofDone,
        proof: tlsnProof,
      });
      setProof(tlsnProof);
      setIsProving(false);
    } catch (e: unknown) {
      await sendMessageToServiceWorker({
        type: ExtensionMessageType.ProofError,
        error: e instanceof Error ? e.message : String(e),
      });
      setIsProving(false);
    }
  }, [provenUrl?.url, formattedHeaders]);

  return (
    <TlsnProofContext.Provider value={{ prove, proof, isProving }}>
      {children}
    </TlsnProofContext.Provider>
  );
};

export const useTlsnProver = () => {
  return useContext(TlsnProofContext);
};
