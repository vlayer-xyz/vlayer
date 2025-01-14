import { ParsedTranscriptData } from "tlsn-js";

const stepAfterColon = 1;

const calculateRequestHeadersRanges = (
  raw: string,
  transcriptRanges: ParsedTranscriptData,
  headers: string[],
) => {
  return headers.map((header) => {
    const header_range = transcriptRanges.headers[header];
    const new_start = raw.indexOf(":", header_range.start) + stepAfterColon;
    return {
      start: new_start,
      end: header_range.end,
    };
  });
};

export { calculateRequestHeadersRanges };
