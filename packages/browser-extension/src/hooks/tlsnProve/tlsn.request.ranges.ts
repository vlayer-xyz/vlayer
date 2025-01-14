import {
  RedactRequestHeaders,
  RedactRequestHeadersExcept,
} from "src/web-proof-commons/types/message";
import { ParsedTranscriptData } from "tlsn-js";
import { CommitData } from "tlsn-js/src/types";
import { match, P } from "ts-pattern";
import { calculateRequestHeadersRanges } from "./tlsn.request.headers.ranges";

function calculateRequestRanges(
  redactionItem: RedactRequestHeadersExcept | RedactRequestHeaders,
  // | RedactRequestUrlQueryExcept
  // | RedactRequestUrlQuery,
  raw: string,
  transcriptRanges: ParsedTranscriptData,
): CommitData[] {
  return match(redactionItem.request)
    .with({ headers: P.array(P.string) }, ({ headers }) => {
      return calculateRequestHeadersRanges(raw, transcriptRanges, headers);
    })
    .with({ headers_except: P.array(P.string) }, ({ headers_except }) => {
      const headers_to_redact = Object.keys(transcriptRanges.headers).filter(
        (header) => !headers_except.includes(header),
      );

      return calculateRequestHeadersRanges(
        raw,
        transcriptRanges,
        headers_to_redact,
      );
    })
    .exhaustive();
}

export { calculateRequestRanges };
