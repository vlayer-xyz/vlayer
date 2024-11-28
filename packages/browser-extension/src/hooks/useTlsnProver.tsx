import {
  NotaryServer,
  Prover as TProver,
  Presentation as TPresentation,
  Transcript,
} from "tlsn-js";
import { Reveal } from "tlsn-wasm";
import React, {
  createContext,
  PropsWithChildren,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";
import * as Comlink from "comlink";
import { formatTlsnHeaders } from "lib/formatTlsnHeaders";
import {
  isDefined,
  ExtensionMessageType,
  WebProofStepNotarize,
} from "../web-proof-commons";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { useProvenUrl } from "./useProvenUrl";
import { useTrackHistory } from "hooks/useTrackHistory";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";

type ProverConfig = {
  serverDns: string;
  maxSentData?: number;
  maxRecvData?: number;
  maxRecvDataOnline?: number;
  deferDecryptionFromStart?: boolean;
};

type PresentationConfig = {
  attestationHex: string;
  secretsHex: string;
  notaryUrl?: string;
  websocketProxyUrl?: string;
  reveal: Reveal;
};

interface TLSNWorker {
  init: (options: { loggingLevel: string }) => Promise<void>;
  Prover: new (config: ProverConfig) => Promise<TProver>;
  Presentation: new (config: PresentationConfig) => Promise<TPresentation>;
}

// tlsn-wasm needs to run in a worker
const worker = new Worker(new URL("./tlsnWorker.ts", import.meta.url), {
  type: "module",
});
const { init, Prover, Presentation } = Comlink.wrap(
  worker,
) as unknown as TLSNWorker;

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
  const [config] = useProvingSessionConfig();

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

      await init({ loggingLevel: "Debug" });
      const notary = NotaryServer.from(provingSessionConfig.notaryUrl || "");
      const prover = await new Prover({
        serverDns: hostname,
        maxSentData: 4096,
        maxRecvData: 16384,
      });

      const sessionUrl = await notary.sessionUrl();
      await prover.setup(sessionUrl);

      await prover.sendRequest(
        provingSessionConfig.wsProxyUrl + `?token=${hostname}`,
        {
          url: provenUrl.url,
          method: "GET",
          headers: {
            "Content-Type": "application/json",
            ...formattedHeaders?.headers,
          },
        },
      );

      const transcript = await prover.transcript();

      console.log("recv", transcript.recv);
      
      const notarizeStep = config.steps.find(
        ({ step }) => step === "notarize",
      ) as WebProofStepNotarize;

      console.log("notarizeStep.jsonRevealPath", notarizeStep.jsonRevealPath);

      const range = getHttpResponseFragments(
        transcript.recv,
        notarizeStep.jsonRevealPath,
      );
      console.log("range", range);

      const commit = {
        sent: [transcript.ranges.sent.all],
        recv: range,
      };

      const notarizationOutputs = await prover.notarize(commit);

      const presentation = await new Presentation({
        attestationHex: notarizationOutputs.attestation,
        secretsHex: notarizationOutputs.secrets,
        notaryUrl: notarizationOutputs.notaryUrl,
        websocketProxyUrl: notarizationOutputs.websocketProxyUrl,
        reveal: commit,
      });

      const tlsnProof = (await presentation.json()) as unknown as string;
      console.log("tlsnProof", tlsnProof.length);
      // const beautyProof = await new Presentation(tlsnProof.data);
      // // const proof = (await new Presentation(
      // //   presentationJSON.data,
      // // )) as TPresentation;
      // // const notary = NotaryServer.from(`http://localhost:7047`);
      // // const notaryKey = await notary.publicKey('hex');
      const verifierOutput = await presentation.verify();

      const beauty = new Transcript({
        sent: verifierOutput?.transcript.sent,
        recv: verifierOutput?.transcript.recv,
      });
      console.log("verifierOutput", tlsnProof);
      console.log("beauty", beauty.recv());
      console.log("beauty", beauty.sent());

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

function getHttpResponseFragments(response: string, jsonPath: string) {
  const headersEndIndex = response.indexOf('\r\n\r\n') + 4;
  const jsonStartIndex = response.indexOf('{', headersEndIndex);
  const jsonEndIndex = response.lastIndexOf('}');
  const jsonBody = response.slice(jsonStartIndex, jsonEndIndex + 1);
  const jsonObject = JSON.parse(jsonBody);
  const getFieldPosition = (obj: any, path: string): { start: number, end: number } => {
    const keys = path.split('.');
    let value = obj;
    for (const key of keys) {
      value = value[key];
    }
    const fieldString = `"${keys[keys.length - 1]}":"${value}"`;
    const fieldStartIndex = response.indexOf(fieldString, jsonStartIndex);
    const fieldEndIndex = fieldStartIndex + fieldString.length;
    return { start: fieldStartIndex, end: fieldEndIndex };
  };
  const fieldPosition = getFieldPosition(jsonObject, jsonPath);
  return [
    {
      start: 0,
      end: headersEndIndex,
    },
    {
      start: jsonStartIndex,
      end: jsonStartIndex + 1,
    },
    {
      start: jsonEndIndex,
      end: jsonEndIndex + 1,
    },
    fieldPosition,
  ];
}

export const useTlsnProver = () => {
  return useContext(TlsnProofContext);
};
