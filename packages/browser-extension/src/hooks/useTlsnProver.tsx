import React, {
  createContext,
  PropsWithChildren,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";
import { formatTlsnHeaders } from "lib/formatTlsnHeaders";
import {
  isDefined,
  ExtensionInternalMessageType,
  getRedactionConfig,
} from "../web-proof-commons";

import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { useProvenUrl } from "./useProvenUrl";
import { useTrackHistory } from "hooks/useTrackHistory";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";
import { LOADING } from "@vlayer/extension-hooks";
import { tlsnProve } from "./tlsnProve/tlsnProve";
import { type Claims } from "lib/types/jwt";
import { validateJwtHostname } from "lib/validateJwtHostname";
import { pipe } from "fp-ts/lib/function";
import { decodeJwt } from "jose";
import { match, P } from "ts-pattern";

const TlsnProofContext = createContext({
  prove: async () => {},
  isProving: false,
  isProvingDone: false,
  error: null as string | null,
  resetTlsnProving: () => {},
});

export const TlsnProofContextProvider = ({ children }: PropsWithChildren) => {
  const [isProving, setIsProving] = useState(false);
  const [isProvingDone, setIsProvingDone] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [formattedHeaders, setFormattedHeaders] = useState<{
    headers: Record<string, string>;
    secretHeaders: string[];
  }>({
    headers: {},
    secretHeaders: [],
  });

  const isProvingReference = useRef(false);

  useEffect(() => {
    isProvingReference.current = isProving;
  }, [isProving]);

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
        type: ExtensionInternalMessageType.ProofProcessing,
        payload: {},
      });
    }, 1000);

    const getWsProxyUrl = (
      baseUrl: string,
      hostname: string,
      token?: string,
    ): string => {
      if (token === undefined) {
        // If no token is specified, we hope for the best, and pass the hostname as token.
        return `${baseUrl}?token=${hostname}`;
      }

      pipe(token, decodeJwt<Claims>, (claims) =>
        validateJwtHostname(claims, hostname),
      );

      return `${baseUrl}?token=${token}`;
    };

    try {
      isDefined(provenUrl?.url, "Missing URL to prove");
      isDefined(provingSessionConfig, "Missing proving session config");
      const hostname = new URL(provenUrl.url).hostname;
      const notaryUrl =
        provingSessionConfig !== LOADING
          ? provingSessionConfig.notaryUrl || ""
          : "";
      const wsProxyUrl =
        provingSessionConfig !== LOADING
          ? getWsProxyUrl(
              provingSessionConfig.wsProxyUrl || "",
              hostname,
              provingSessionConfig.token,
            )
          : "";

      const redactionConfig = match(provingSessionConfig)
        .with(LOADING, () => [])
        .with(P.nullish, () => [])
        .otherwise((w) => getRedactionConfig(w));
      const tlsnProof = await tlsnProve(
        notaryUrl,
        hostname,
        wsProxyUrl,
        provenUrl.url,
        provenUrl.method,
        formattedHeaders,
        redactionConfig,
        provenUrl.body,
      );
      setIsProvingDone(true);
      // mutable ref is need here to avoid stale closure
      if (isProvingReference.current === false) {
        return;
      }

      void sendMessageToServiceWorker({
        type: ExtensionInternalMessageType.ProofDone,
        payload: {
          ...tlsnProof,
        },
      });
      //only set proof is is proving ( session was not reset)
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e));
      void sendMessageToServiceWorker({
        type: ExtensionInternalMessageType.ProofError,
        payload: {
          error: e instanceof Error ? e.message : String(e),
        },
      });
    } finally {
      setIsProving(false);
      clearInterval(progressInterval);
    }
  }, [provenUrl, formattedHeaders, provingSessionConfig]);

  const resetTlsnProving = useCallback(() => {
    setIsProvingDone(false);
    setIsProving(false);
    setError(null);
  }, []);

  return (
    <TlsnProofContext.Provider
      value={{
        prove,
        isProving,
        isProvingDone,
        error,
        resetTlsnProving,
      }}
    >
      {children}
    </TlsnProofContext.Provider>
  );
};

export const useTlsnProver = () => {
  return useContext(TlsnProofContext);
};
