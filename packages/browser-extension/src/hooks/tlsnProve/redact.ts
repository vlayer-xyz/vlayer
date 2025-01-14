import { Commit } from "tlsn-wasm";
import { match, P } from "ts-pattern";
import { calculateRequestRanges } from "./tlsn.request.ranges";
import { calculateResponseRanges } from "./tlsn.response.ranges";
import { RedactionConfig } from "src/web-proof-commons/types/message";
import { ParsedTranscriptData } from "tlsn-js";
export type Transcript = {
  sent: string;
  recv: string;
  ranges: {
    recv: ParsedTranscriptData;
    sent: ParsedTranscriptData;
  };
};

const emptyCommit: Commit = {
  sent: [],
  recv: [],
};

export function redact(
  transcript: Transcript,
  redactionConfig: RedactionConfig,
) {
  return redactionConfig.reduce((commit, redactionItem) => {
    return match(redactionItem)
      .with({ response: P.any }, (responseRedactionItem) => {
        return {
          ...commit,
          recv: [
            ...commit.recv,
            ...calculateResponseRanges(
              responseRedactionItem,
              transcript.recv,
              transcript.ranges.recv,
            ),
          ],
        };
      })
      .with({ request: P.any }, (requestRedactionItem) => {
        return {
          ...commit,
          sent: [
            ...commit.sent,
            ...calculateRequestRanges(
              requestRedactionItem,
              transcript.sent,
              transcript.ranges.sent,
            ),
          ],
        };
      })
      .exhaustive();
  }, emptyCommit);
}
