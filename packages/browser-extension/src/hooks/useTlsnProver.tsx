import React, {
  createContext,
  PropsWithChildren,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";
import { formatTlsnHeaders } from "lib/formatTlsnHeaders";
import {
  isDefined,
  ExtensionMessageType,
  getRedactionConfig,
} from "../web-proof-commons";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { useProvenUrl } from "./useProvenUrl";
import { useTrackHistory } from "hooks/useTrackHistory";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";
import { LOADING } from "@vlayer/extension-hooks";
import { tlsnProve } from "./tlsnProve/tlsnProve";
import { z } from "zod";

const claimsSchema = z.object({
  host: z.string(),
  port: z.number(),
  sub: z.string(),
});

const TlsnProofContext = createContext({
  prove: async () => {},
  proof: null as object | null,
  isProving: false,
  error: null as string | null,
});

export const TlsnProofContextProvider = ({ children }: PropsWithChildren) => {
  const [proof, setProof] = useState<object | null>(null);
  const [isProving, setIsProving] = useState(false);
  const [error, setError] = useState<string | null>(null);
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

    const getWsProxyUrl = (
      baseUrl: string,
      hostname: string,
      jwtToken: string | null,
    ): string => {
      if (jwtToken === null) {
        return baseUrl + `?token=${hostname}`;
      }
      const payload = jwtToken.split(".")[1] ?? "";
      const claims = Buffer.from(payload, "base64").toString("utf8");
      const parsed = claimsSchema.safeParse(JSON.parse(claims));
      if (parsed.success) {
        if (parsed.data?.host === hostname) {
          return baseUrl + `?token=${jwtToken}`;
        }
        throw Error(
          `Invalid JWT token: token valid for hostname ${parsed.data?.host}, but needs ${hostname}`,
        );
      }
      throw Error("Invalid JWT token");
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
              provingSessionConfig.jwtToken,
            )
          : "";

      const redactionConfig =
        provingSessionConfig !== LOADING
          ? getRedactionConfig(provingSessionConfig)
          : [];

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

      void sendMessageToServiceWorker({
        type: ExtensionMessageType.ProofDone,
        payload: {
          ...tlsnProof,
        },
      });

      setProof(tlsnProof);
    } catch (e: unknown) {
      setError(e instanceof Error ? e.message : String(e));
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
    <TlsnProofContext.Provider value={{ prove, proof, isProving, error }}>
      {children}
    </TlsnProofContext.Provider>
  );
};

export const useTlsnProver = () => {
  return useContext(TlsnProofContext);
};
