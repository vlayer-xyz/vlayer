import {
  RedactResponseHeaders,
  RedactResponseHeadersExcept,
  RedactResponseJsonBody,
  RedactResponseJsonBodyExcept,
} from "src/web-proof-commons";
import { CommitData } from "tlsn-js/src/types";
import { match, P } from "ts-pattern";
import { getStringPaths } from "./utils";
import {
  filterExceptHeaders,
  calculateHeadersRanges,
  getAllHeaders,
} from "./headers";
import { calculateJsonBodyRanges, filterExceptPaths } from "./body";
import { MessageTranscript } from "./types";

export const calculateResponseRanges = (
  redactionItem:
    | RedactResponseHeaders
    | RedactResponseHeadersExcept
    | RedactResponseJsonBody
    | RedactResponseJsonBodyExcept,
  transcript: MessageTranscript,
): CommitData[] => {
  return (
    match(redactionItem.response)
      // Headers
      .with({ headers: P.array(P.string) }, ({ headers: headersToRedact }) => {
        return calculateHeadersRanges(transcript.message, headersToRedact);
      })
      // Headers except specified
      .with(
        { headers_except: P.array(P.string) },
        ({ headers_except: headersToExcludeFromRedaction }) => {
          // Filter out the headers that are in the headers_except array
          const filteredHeaders = filterExceptHeaders(
            headersToExcludeFromRedaction,
            getAllHeaders(transcript.headers),
          );
          return calculateHeadersRanges(transcript.message, filteredHeaders);
        },
      )
      // Json body
      .with({ json_body: P.array(P.string) }, ({ json_body }) => {
        return calculateJsonBodyRanges(transcript, json_body);
      })
      // Json body except specified paths
      .with({ json_body_except: P.array(P.string) }, ({ json_body_except }) => {
        // Get all string paths in the json body
        const paths = getStringPaths(transcript.body.content.toString());
        // Filter out the paths that are in the json_body_except array
        const filteredPaths = filterExceptPaths(json_body_except, paths);
        // Calculate the ranges for the filtered paths
        return calculateJsonBodyRanges(transcript, filteredPaths);
      })
      .exhaustive()
  );
};
