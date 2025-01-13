import { ParsedTranscriptData } from "tlsn-js";
import {
  RedactResponseHeaders,
  RedactResponseHeadersExcept,
  RedactRequestHeadersExcept,
  RedactRequestHeaders,
  RedactRequestUrlQuery,
  RedactRequestUrlQueryExcept,
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
function calculateRequestRanges(
  redactionItem:
    | RedactRequestHeadersExcept
    | RedactRequestHeaders
    | RedactRequestUrlQueryExcept
    | RedactRequestUrlQuery,
  raw: string,
  transcriptRanges: ParsedTranscriptData,
): CommitData[] {
  const headers_ranges = transcriptRanges.headers;

  return match(redactionItem.request)
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
    .with({ url_query: P.array(P.string) }, ({ url_query }) => {
      return url_query.map((query) => {
        return count_url_query(raw, query);
      });
    })
    .with({ url_query_except: P.array(P.string) }, ({ url_query_except }) => {
      const queries_to_redact = find_all_url_queries(raw).filter(
        (query) => !url_query_except.includes(query),
      );

      return queries_to_redact.map((query) => {
        return count_url_query(raw, query);
      });
    })
    .exhaustive();
}

function count_url_query(raw: string, query: string,): CommitData {
  const stepOverFirstAmpersand = 1;
  const start = raw.indexOf("&" + query + "=") + stepOverFirstAmpersand;
  const secondAmpersandPosition = raw.indexOf("&", start);
  const end = secondAmpersandPosition !== -1 ? secondAmpersandPosition : raw.indexOf(" ", start);
  return {
    start,
    end,
  };
}

function find_all_url_queries(raw: string): string[] {
  const url_queries = raw.match(/&\w+=/g);
  return url_queries ? url_queries.map((query) => query.slice(1, -1)) : [];
}

export { calculateResponseRanges, calculateRequestRanges };
