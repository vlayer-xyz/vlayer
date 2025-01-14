import {
  RedactRequestHeaders,
  RedactRequestHeadersExcept,
  RedactRequestUrlQueryParam,
  RedactRequestUrlQueryParamExcept,
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
    | RedactRequestUrlQueryParamExcept
    | RedactRequestUrlQueryParam,
  raw: string,
  transcriptRanges: ParsedTranscriptData,
): CommitData[] {
  const url = findUrlInRequest(raw, transcriptRanges.info);
  const url_offset = raw.indexOf(url);

  return match(redactionItem.request)
    .with({ headers: P.array(P.string) }, ({ headers }) => {
      return calculateRequestHeadersRanges(raw, transcriptRanges, headers);
    })
    .with({ headers_except: P.array(P.string) }, ({ headers_except }) => {
      const headersToRedact = Object.keys(transcriptRanges.headers).filter(
        (header) => !headers_except.includes(header),
      );

      return calculateRequestHeadersRanges(
        raw,
        transcriptRanges,
        headersToRedact,
      );
    })
    .with({ url_query: P.array(P.string) }, ({ url_query }) => {
      return calculateRequestQueriesRanges(url_query, url, url_offset);
    })
    .with({ url_query_except: P.array(P.string) }, ({ url_query_except }) => {
      const queriesToRedact = findAllUrlQueries(url).filter(
        (query) => !url_query_except.includes(query),
      );

      return calculateRequestQueriesRanges(queriesToRedact, url, url_offset);
    })
    .exhaustive();
}

function findUrlInRequest(raw: string, request_range: CommitData): string {
  const request = raw.slice(request_range.start, request_range.end);
  const url = request.split(" ")[1]; // METHOD PATH PROTOCOL_VERSION
  return url;
}

function findAllUrlQueries(path: string): string[] {
  const url = new URL(path);

  const params: string[] = [];
  url.searchParams.forEach((_, key) => {
    params.push(key);
  });
  return params;
}

export { calculateRequestRanges };
