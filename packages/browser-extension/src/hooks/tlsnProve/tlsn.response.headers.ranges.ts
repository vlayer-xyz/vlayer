import { ParsedTranscriptData } from "tlsn-js";
const stepAfterColon = 1;

const filterExceptHeaders = (except: string[], headers: string[]) => {
  return headers.filter((header) => !except.includes(header));
};

const calculateHeadersRanges = (
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

export { calculateHeadersRanges, filterExceptHeaders };
