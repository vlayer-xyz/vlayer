import { NotaryServer, Transcript } from "tlsn-js";
import { wrap } from "comlink";
import { Prover as TProver, Presentation as TPresentation } from "tlsn-js";
import type { PresentationJSON } from "tlsn-js/src/types";
import { Reveal, Method } from "tlsn-wasm";
import { type RedactionConfig } from "../../web-proof-commons";
import { redact } from "./redaction/redact";
import { HTTPMethod } from "lib/HttpMethods";
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
const { init, Prover, Presentation } = wrap(worker) as unknown as TLSNWorker;

export async function tlsnProve(
  notaryUrl: string,
  hostname: string,
  wsProxyUrl: string,
  notarizeRequestUrl: string,
  method: HTTPMethod = HTTPMethod.GET,
  formattedHeaders: {
    headers: Record<string, string>;
    secretHeaders: string[];
  },
  redactionConfig: RedactionConfig,
  requestBody?: string,
): Promise<{
  presentationJson: PresentationJSON;
  decodedTranscript: {
    sent: string;
    recv: string;
  };
}> {
  await init({ loggingLevel: "Debug" });
  const notary = NotaryServer.from(notaryUrl);
  const prover = await new Prover({
    serverDns: hostname,
    maxSentData: 4096,
    maxRecvData: 16384,
  });

  const sessionUrl = await notary.sessionUrl();
  await prover.setup(sessionUrl);
  const request = {
    url: notarizeRequestUrl,
    method: method as Method,
    headers: {
      ...formattedHeaders?.headers,
      "Content-Type": "application/json",
    },
    body: requestBody,
  };

  const res = await prover.sendRequest(wsProxyUrl, request);

  if (res.status < 200 || res.status >= 300) {
    throw new Error("Authentication failed. Please restart the process.");
  }

  console.log("Received response", res);

  const transcript = await prover.transcript();

  console.log("Transcript", transcript);

  const commit = redact(transcript, redactionConfig);

  console.log("Commit", commit);
  const notarizationOutputs = await prover.notarize(commit);

  const presentation = await new Presentation({
    attestationHex: notarizationOutputs.attestation,
    secretsHex: notarizationOutputs.secrets,
    notaryUrl: notarizationOutputs.notaryUrl,
    websocketProxyUrl: notarizationOutputs.websocketProxyUrl,
    reveal: commit,
  });

  const presentationJson = await presentation.json();
  const decodedProof = await presentation.verify();
  console.log("Decoded proof", decodedProof);

  const decodedTranscript = new Transcript({
    sent: decodedProof?.transcript.sent,
    recv: decodedProof?.transcript.recv,
  });
  console.log("Decoded transcript", decodedTranscript);
  return {
    presentationJson,
    decodedTranscript: {
      sent: decodedTranscript.sent(),
      recv: decodedTranscript.recv(),
    },
  };
}
