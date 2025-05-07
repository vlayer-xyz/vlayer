import { NotaryServer, Transcript } from "tlsn-js";
import { wrap } from "comlink";
import { Prover as TProver, Presentation as TPresentation } from "tlsn-js";
import type { PresentationJSON } from "tlsn-js/src/types";
import { Reveal, Method } from "tlsn-wasm";
import {
  TlsnProveError,
  TlsnProveNon200ResponseError,
  type RedactionConfig,
} from "../../web-proof-commons";

import { redact } from "./redaction/redact";
import { HTTPMethod } from "lib/HttpMethods";
import debug from "debug";
const log = debug("extension:tlsnProve");
import {
  calculateRequestSize,
  DEFAULT_MAX_RECV_DATA,
} from "./requestSizeCalculator";

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
  try {
    // tlsn-wasm needs to run in a worker
    const worker = new Worker(new URL("./tlsnWorker.ts", import.meta.url), {
      type: "module",
    });
    const { init, Prover, Presentation } = wrap(
      worker,
    ) as unknown as TLSNWorker;

    await init({ loggingLevel: "Debug" });
    const notary = NotaryServer.from(notaryUrl);

    const request = {
      url: notarizeRequestUrl,
      method: method as Method,
      headers: formattedHeaders?.headers,
      body: requestBody,
    };
    log("request size is gonna be", calculateRequestSize(request));
    const prover = await new Prover({
      serverDns: hostname,
      maxSentData: calculateRequestSize(request),
      maxRecvData: DEFAULT_MAX_RECV_DATA,
    });

    const sessionUrl = await notary.sessionUrl();
    await prover.setup(sessionUrl);

    const res = await prover.sendRequest(wsProxyUrl, request);
    if (res.status < 200 || res.status >= 300) {
      throw new TlsnProveNon200ResponseError();
    }
    const proverTranscript = await prover.transcript();

    const transcript = {
      recv: new TextDecoder().decode(new Uint8Array(proverTranscript.recv)),
      sent: new TextDecoder().decode(new Uint8Array(proverTranscript.sent)),
    };

    log("Transcript", transcript);

    const commit = redact(transcript, redactionConfig);

    log("Commit", commit);
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
    log("Decoded proof", decodedProof);

    const decodedTranscript = new Transcript({
      sent: decodedProof?.transcript.sent,
      recv: decodedProof?.transcript.recv,
    });
    log("Decoded transcript", decodedTranscript);

    return {
      presentationJson,
      decodedTranscript: {
        sent: decodedTranscript.sent(),
        recv: decodedTranscript.recv(),
      },
    };
  } catch (e) {
    log("Error while proving TLSN", e);
    if (e instanceof TlsnProveNon200ResponseError) {
      throw e;
    }
    throw new TlsnProveError({
      message: "An error occurred while proving TLSN",
      name: "TLSN_PROVE_ERROR",
    });
  }
}
