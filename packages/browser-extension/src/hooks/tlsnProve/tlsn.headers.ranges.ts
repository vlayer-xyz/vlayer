import { HeaderNotFoundError } from "./tlsn.ranges.error";
import { MessagePartTranscript } from "./utils";
const stepAfterColon = 2;

const filterExceptHeaders = (except: string[], headers: string[]) => {
  const filteredHeaders = headers.filter((header) => !except.includes(header));
  return filteredHeaders;
};

const calculateHeadersRanges = (
  transcript: MessagePartTranscript,
  headers: string[],
) => {
  return headers.map((header) => {
    const headerStart =
      transcript.content.indexOf(header) + transcript.range.start;
    if (headerStart === -1) {
      throw new HeaderNotFoundError(header);
    }
    const colonIndex = transcript.content.indexOf(":", headerStart);
    const valueStart = colonIndex + stepAfterColon;
    const nextNewline = transcript.content.indexOf("\r\n", valueStart);
    const valueEnd =
      nextNewline === -1 ? transcript.content.length : nextNewline;
    return {
      start: valueStart,
      end: valueEnd,
    };
  });
};

export const getAllHeaders = (transcript: MessagePartTranscript) => {
  const headers = transcript.content
    .split("\r\n")
    .filter((line) => line.includes(":"))
    .map((line) => line.split(":")[0])
    .map((header) => header.toUtf16String());
  return headers;
};

export { calculateHeadersRanges, filterExceptHeaders };
