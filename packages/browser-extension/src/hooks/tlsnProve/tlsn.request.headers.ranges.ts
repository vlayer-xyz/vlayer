import { ParsedTranscriptData } from "tlsn-js";

const stepAfterColon = 1;

const calculateRequestHeadersRanges = (
  raw: string,
  transcriptRanges: ParsedTranscriptData,
  headers: string[],
) => {
  return headers.map((header) => {
    const headerRange = transcriptRanges.headers[header];
    const newStart = raw.indexOf(":", headerRange.start) + stepAfterColon;
    return {
      start: newStart,
      end: headerRange.end,
    };
  });
};

export { calculateRequestHeadersRanges };
