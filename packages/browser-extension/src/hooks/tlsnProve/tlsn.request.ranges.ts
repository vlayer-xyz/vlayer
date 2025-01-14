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
  transcriptRanges: ParsedTranscriptData
): CommitData[] {
  return match(redactionItem.request)
    .with({ headers: P.array(P.string) }, ({ headers }) => {
      return calculateRequestHeadersRanges(raw, transcriptRanges, headers);
    })
    .with({ headers_except: P.array(P.string) }, ({ headers_except }) => {
      const headersToRedact = Object.keys(transcriptRanges.headers).filter(
        (header) => !headers_except.includes(header)
      );

      return calculateRequestHeadersRanges(
        raw,
        transcriptRanges,
        headersToRedact
      );
    })
    .with({ url_query: P.array(P.string) }, ({ url_query }) => {
      return calculateRequestQueriesRanges(url_query, raw);
    })
    .with({ url_query_except: P.array(P.string) }, ({ url_query_except }) => {
      const queriesToRedact = findAllUrlQueries(raw).filter(
        (query) => !url_query_except.includes(query)
      );

      return calculateRequestQueriesRanges(queriesToRedact, raw);
    })
    .exhaustive();
}

function findAllUrlQueries(raw: string): string[] {
  const urlQueries = raw.match(/&\w+=/g);
  return urlQueries ? urlQueries.map((query) => query.slice(1, -1)) : [];
}

export { calculateRequestRanges };
