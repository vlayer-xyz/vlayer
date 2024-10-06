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
  useTrackHistory();
  const [proof, setProof] = useState<object | null>(null);
  const [isProving, setIsProving] = useState(false);
  const [formattedHeaders, setFormattedHeaders] = useState<{
    headers: Record<string, string>;
    secretHeaders: string[];
  }>({
    headers: {},
    secretHeaders: [],
  });

  const provenUrl = useProvenUrl();

  useEffect(() => {
    setFormattedHeaders(
      formatTlsnHeaders(provenUrl?.headers ?? [], provenUrl?.cookies ?? []),
    );
  }, []);

  const prove = useCallback(async () => {
    setIsProving(true);

    const [provingSessionConfig] = useProvingSessionConfig();

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
      browser.runtime.sendMessage({
        type: ExtensionMessage.ProofDone,
        proof: tlsnProof,
      });
      setProof(proof);
      setIsProving(false);
    } catch (e) {
      console.error("error in tlsnotary", e);
      browser.runtime.sendMessage({
        type: ExtensionMessage.ProofError,
        error: e,
      });
      setIsProving(false);
    }
  }, []);

  return (
    <TlsnProofContext.Provider value={{ prove, proof, isProving }}>
      {children}
    </TlsnProofContext.Provider>
  );
};

export const useTlsnProver = () => {
  return useContext(TlsnProofContext);
};
