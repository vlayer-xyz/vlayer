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
  prove: () => {},
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
  // hook history and config into provider
  // TODO : consider renaming parent component as it makes more than just tlsn proof provider
  useTrackHistory();
  const [provingSessionConfig] = useProvingSessionConfig();
  const provenUrl = useProvenUrl();

  // format headers to make it accepted by tlsn js api
  useEffect(() => {
    setFormattedHeaders(
      formatTlsnHeaders(provenUrl?.headers ?? [], provenUrl?.cookies ?? []),
    );
  }, [provenUrl]);

  const prove = useCallback(async () => {
    setIsProving(true);

    try {
      //TODO : make sure on hooks level its defined
      isDefined(provenUrl?.url, "Missing URL to prove");
      isDefined(provingSessionConfig, "Missing proving session config");

      const tlsnProof = await tlsnProve(removeQueryParams(provenUrl?.url), {
        notaryUrl: provingSessionConfig.notaryUrl,
        websocketProxyUrl: `${provingSessionConfig.wsProxyUrl}?token=${new URL(provenUrl.url).host}`,
        method: "GET",
        headers: formattedHeaders?.headers,
        secretHeaders: formattedHeaders?.secretHeaders,
      });
      // let service worker know proof is done
      console.log("sending proof to background", tlsnProof);
      sendMessageToServiceWorker({
        type: ExtensionMessageType.ProofDone,
        proof: tlsnProof,
      });
      setProof(tlsnProof);
      setIsProving(false);
    } catch (e: unknown) {
      console.error("error in tlsnotary", e);
      sendMessageToServiceWorker({
        type: ExtensionMessageType.ProofError,
        error: e instanceof Error ? e.message : String(e),
      });
      setIsProving(false);
    }
  }, [provenUrl, formattedHeaders]);

  return (
    <TlsnProofContext.Provider value={{ prove, proof, isProving }}>
      {children}
    </TlsnProofContext.Provider>
  );
};

export const useTlsnProver = () => {
  return useContext(TlsnProofContext);
};
