import {
  RedactRequestHeaders,
  RedactRequestHeadersExcept,
  RedactRequestUrlQuery,
  RedactRequestUrlQueryExcept,
} from "src/web-proof-commons/types/message";
import { ParsedTranscriptData } from "tlsn-js";
import { CommitData } from "tlsn-js/src/types";
import { match, P } from "ts-pattern";
import { calculateRequestHeadersRanges } from "./tlsn.request.headers.ranges";
import { calculateRequestQueriesRanges } from "./tlsn.request.queries.ranges";

function calculateRequestRanges(
  redactionItem:
    | RedactRequestHeadersExcept
    | RedactRequestHeaders
    | RedactRequestUrlQueryExcept
    | RedactRequestUrlQuery,
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
    .with({ url_query: P.array(P.string) }, ({ url_query }) => {
      return calculateRequestQueriesRanges(url_query, raw);
    })
    .with({ url_query_except: P.array(P.string) }, ({ url_query_except }) => {
      const queries_to_redact = find_all_url_queries(raw).filter(
        (query) => !url_query_except.includes(query),
      );

      return calculateRequestQueriesRanges(queries_to_redact, raw);
    })
    .exhaustive();
}

function find_all_url_queries(raw: string): string[] {
  const url_queries = raw.match(/&\w+=/g);
  return url_queries ? url_queries.map((query) => query.slice(1, -1)) : [];
}

export { calculateRequestRanges };
