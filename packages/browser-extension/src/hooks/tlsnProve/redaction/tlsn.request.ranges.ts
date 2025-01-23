import {
  RedactRequestHeaders,
  RedactRequestHeadersExcept,
  RedactRequestUrlQueryParam,
  RedactRequestUrlQueryParamExcept,
} from "src/web-proof-commons";
import { CommitData } from "tlsn-js/src/types";
import { match, P } from "ts-pattern";
import {
  calculateHeadersRanges,
  filterExceptHeaders,
  getAllHeaders,
} from "./headers";
import {
  calculateRequestQueryParamsRanges,
  calculateRequestQueryParamsRangesExcept,
} from "./query";
import { findUrlInRequest } from "./utils";
import { MessageTranscript } from "./types";
export const calculateRequestRanges = (
  redactionItem:
    | RedactRequestHeadersExcept
    | RedactRequestHeaders
    | RedactRequestUrlQueryParam
    | RedactRequestUrlQueryParamExcept,
  transcript: MessageTranscript,
): CommitData[] => {
  const { url, url_offset } = findUrlInRequest(transcript);

  return match(redactionItem.request)
    .with({ headers: P.array(P.string) }, ({ headers: headersToRedact }) => {
      return calculateHeadersRanges(transcript.message, headersToRedact);
    })
    .with(
      { headers_except: P.array(P.string) },
      ({ headers_except: headersToExcludeFromRedaction }) => {
        const filteredHeaders = filterExceptHeaders(
          headersToExcludeFromRedaction,
          getAllHeaders(transcript.headers),
        );
        return calculateHeadersRanges(transcript.message, filteredHeaders);
      },
    )
    .with({ url_query: P.array(P.string) }, ({ url_query }) => {
      return calculateRequestQueryParamsRanges(url_query, url, url_offset);
    })
    .with({ url_query_except: P.array(P.string) }, ({ url_query_except }) => {
      return calculateRequestQueryParamsRangesExcept(
        url_query_except,
        url,
        url_offset,
      );
    })
    .exhaustive();
};
