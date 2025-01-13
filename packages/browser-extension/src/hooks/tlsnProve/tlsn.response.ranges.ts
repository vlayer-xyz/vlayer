import { ParsedTranscriptData } from "tlsn-js";
import {
  RedactResponseHeaders,
  RedactResponseHeadersExcept,
  RedactResponseJsonBody,
  RedactResponseJsonBodyExcept,
} from "../../web-proof-commons/types/message";
import { CommitData } from "tlsn-js/src/types";
import { match, P } from "ts-pattern";
import { getStringPaths } from "./getStringPaths";
import {
  filterExceptHeaders,
  calculateHeadersRanges,
} from "./tlsn.response.headers.ranges";
import {
  calculateJsonBodyRanges,
  filterExceptPaths,
} from "./tlsn.response.body.ranges";

export const calculateResponseRanges = (
  redactionItem:
    | RedactResponseHeaders
    | RedactResponseHeadersExcept
    | RedactResponseJsonBody
    | RedactResponseJsonBodyExcept,
  raw: string,
  transcriptRanges: ParsedTranscriptData,
): CommitData[] => {
  return (
    match(redactionItem.response)
      // Headers
      .with({ headers: P.array(P.string) }, ({ headers }) => {
        return calculateHeadersRanges(raw, transcriptRanges, headers);
      })
      // Headers except specified
      .with({ headers_except: P.array(P.string) }, ({ headers_except }) => {
        // Filter out the headers that are in the headers_except array
        const filteredHeaders = filterExceptHeaders(
          headers_except,
          Object.keys(transcriptRanges.headers),
        );
        return calculateHeadersRanges(raw, transcriptRanges, filteredHeaders);
      })
      // Json body
      .with({ json_body: P.array(P.string) }, ({ json_body }) => {
        return calculateJsonBodyRanges(raw, transcriptRanges, json_body);
      })
      // Json body except specified paths
      .with({ json_body_except: P.array(P.string) }, ({ json_body_except }) => {
        // Get all string paths in the json body
        const paths = getStringPaths(raw);
        // Filter out the paths that are in the json_body_except array
        const filteredPaths = filterExceptPaths(json_body_except, paths);
        // Calculate the ranges for the filtered paths
        return calculateJsonBodyRanges(raw, transcriptRanges, filteredPaths);
      })
      .exhaustive()
  );
};
