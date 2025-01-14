import { ParsedTranscriptData } from "tlsn-js";
import {
  RedactResponseHeaders,
  RedactResponseHeadersExcept,
} from "../../web-proof-commons/types/message";
import { CommitData } from "tlsn-js/src/types";
import { match, P } from "ts-pattern";
const stepAfterColon = 1;

function calculateResponseRanges(
  redactionItem: RedactResponseHeaders | RedactResponseHeadersExcept,
  // | RedactResponseJsonBody
  // | RedactResponseJsonBodyExcept,
  raw: string,
  transcriptRanges: ParsedTranscriptData,
): CommitData[] {
  const headers_ranges = transcriptRanges.headers;

  return match(redactionItem.response)
    .with({ headers: P.array(P.string) }, ({ headers }) => {
      return headers.map((header) => {
        const header_range = headers_ranges[header];
        const new_start = raw.indexOf(":", header_range.start) + stepAfterColon;
        return {
          start: new_start,
          end: header_range.end,
        };
      });
    })
    .with({ headers_except: P.array(P.string) }, ({ headers_except }) => {
      const headers_to_redact = Object.keys(transcriptRanges.headers).filter(
        (header) => !headers_except.includes(header),
      );
      return headers_to_redact.map((header) => {
        const header_range = headers_ranges[header];
        const new_start = raw.indexOf(":", header_range.start) + stepAfterColon;
        return {
          start: new_start,
          end: header_range.end,
        };
      });
    })
    .exhaustive();
}

export { calculateResponseRanges };
