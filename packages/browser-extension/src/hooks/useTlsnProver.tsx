import { NotaryServer } from 'tlsn-js';
import React, {
  createContext,
  PropsWithChildren,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";
import * as Comlink from 'comlink';
import { formatTlsnHeaders } from "lib/formatTlsnHeaders";
import { isDefined, ExtensionMessageType } from "../web-proof-commons";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { useProvenUrl } from "./useProvenUrl";
import { useTrackHistory } from "hooks/useTrackHistory";
import { removeQueryParams } from "lib/removeQueryParams";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";

const worker = new Worker(new URL('./worker.ts', import.meta.url), { type: 'module' });
const { init, Prover, Presentation }: any = Comlink.wrap(worker); 

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

      const hostname = new URL(provenUrl.url).hostname;

      await init({ loggingLevel: 'Debug'});
      const notary = NotaryServer.from(provingSessionConfig.notaryUrl || "");
      const prover = await new Prover({ 
        serverDns: hostname,
        maxSentData: 4096,
        maxRecvData: 16384,
      });

      const sessionUrl = await notary.sessionUrl();
      await prover.setup(sessionUrl);

      const res = await prover.sendRequest(provingSessionConfig.wsProxyUrl + `?token=${hostname}`, {
        url: removeQueryParams(provenUrl.url),
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
          ...formattedHeaders?.headers,
        },
      });

      console.log("Received response", res);

      const transcript = await prover.transcript();
      const commit = {
        sent: [
          transcript.ranges.sent.all,
        ],
        recv: [
          transcript.ranges.recv.all,
        ],
      };
      const notarizationOutputs = await prover.notarize(commit);

      const presentation = (await new Presentation({
        attestationHex: notarizationOutputs.attestation,
        secretsHex: notarizationOutputs.secrets,
        notaryUrl: notarizationOutputs.notaryUrl,
        websocketProxyUrl: notarizationOutputs.websocketProxyUrl,
        reveal: commit,
      }));

      const tlsnProof = await presentation.json();

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
