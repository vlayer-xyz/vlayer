import { NotaryServer } from "tlsn-js";
import { wrap } from "comlink";
import { Prover as TProver, Presentation as TPresentation } from "tlsn-js";
import type { PresentationJSON } from "tlsn-js/src/types";
import { Reveal } from "tlsn-wasm";

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
  formattedHeaders: {
    headers: Record<string, string>;
    secretHeaders: string[];
  },
): Promise<PresentationJSON> {
  await init({ loggingLevel: "Debug" });
  const notary = NotaryServer.from(notaryUrl);
  const prover = await new Prover({
    serverDns: hostname,
    maxSentData: 4096,
    maxRecvData: 16384,
  });

  const sessionUrl = await notary.sessionUrl();
  await prover.setup(sessionUrl);

  const res = await prover.sendRequest(wsProxyUrl, {
    url: notarizeRequestUrl,
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      ...formattedHeaders?.headers,
    },
  });

  console.log("Received response", res);

  const transcript = await prover.transcript();
  const commit = {
    sent: [transcript.ranges.sent.all],
    recv: [transcript.ranges.recv.all],
  };
  const notarizationOutputs = await prover.notarize(commit);

  const presentation = await new Presentation({
    attestationHex: notarizationOutputs.attestation,
    secretsHex: notarizationOutputs.secrets,
    notaryUrl: notarizationOutputs.notaryUrl,
    websocketProxyUrl: notarizationOutputs.websocketProxyUrl,
    reveal: commit,
  });

  return await presentation.json();
}

type Transcript = {
  sent: string;
  recv: string;
  ranges: {
    recv: ParsedTranscriptData;
    sent: ParsedTranscriptData;
  };
}

function redact(transcript: Transcript, _redactionConfig: RedactionConfig): Commit {
  return {
    sent: [transcript.ranges.sent.all],
    recv: [transcript.ranges.recv.all],
  };
}
