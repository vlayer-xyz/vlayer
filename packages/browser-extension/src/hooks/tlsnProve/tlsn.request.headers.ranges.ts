import { ParsedTranscriptData } from "tlsn-js";
import { HeaderNotFound } from "./tlsn.ranges.error";

const stepAfterColon = 1;

const calculateRequestHeadersRanges = (
  raw: string,
  transcriptRanges: ParsedTranscriptData,
  headers: string[],
) => {
  return headers.map((header) => {
    const headerRange = Object.entries(transcriptRanges.headers).find(
      ([key]) => key.toLowerCase() === header.toLowerCase(),
    );
    if (!headerRange) {
      throw new HeaderNotFound(header);
    }

    const newStart = raw.indexOf(":", headerRange[1].start) + stepAfterColon;
    return {
      start: newStart,
      end: headerRange[1].end,
    };
  });
};

export { calculateRequestHeadersRanges };
