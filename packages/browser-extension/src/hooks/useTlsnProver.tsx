import { prove as tlsnProve } from "tlsn-js";
import browser from "webextension-polyfill";
import React, {
  useContext,
  createContext,
  useCallback,
  useState,
  PropsWithChildren,
  useEffect,
} from "react";
import { formatTlsnHeaders } from "../lib/formatTlsnHeaders";
import { ExtensionMessage } from "@vlayer/web-proof-commons/constants/message";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { useProvenUrl } from "./useProvenUrl";
import { useTrackHistory } from "hooks/useTrackHistory";

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
  }, [provenUrl?.headers, provenUrl?.cookies]);

  const prove = useCallback(async () => {
    setIsProving(true);
    try {
      //TODO : make sure on hooks level its defined
      if (!provenUrl?.url) {
        throw new Error("Missing URL to proove");
      }
      const tlsnProof = await tlsnProve(provenUrl?.url, {
        notaryUrl: provingSessionConfig.notaryUrl,
        websocketProxyUrl: provingSessionConfig.wsProxyUrl,
        method: "GET",
        headers: formattedHeaders?.headers,
        secretHeaders: formattedHeaders?.secretHeaders,
      });

      // let service worker know proof is done
      browser.runtime.sendMessage({
        type: ExtensionMessage.ProofDone,
        proof: tlsnProof,
      });
      setProof(tlsnProof);
      setIsProving(false);
    } catch (e) {
      browser.runtime.sendMessage({
        type: ExtensionMessage.ProofError,
        error: e,
      });
      setIsProving(false);
    }
  }, [provenUrl?.url]);

  return (
    <TlsnProofContext.Provider value={{ prove, proof, isProving }}>
      {children}
    </TlsnProofContext.Provider>
  );
};

export const useTlsnProver = () => {
  return useContext(TlsnProofContext);
};
