import { HeaderNotFoundError } from "../utils/error";
import { MessagePartTranscript, MessageTranscript } from "../types";
import { pipe } from "fp-ts/lib/function";
import { EncodedString } from "../utils/encodeString/EncodedString";
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
    const headerStartInRange = transcript.content.caseInsensitiveIndexOf(
      `\r\n${header}:`,
    );

    if (headerStartInRange === -1) {
      throw new HeaderNotFoundError(header);
    }
    const headerStart =
      headerStartInRange +
      transcript.range.start +
      new EncodedString(`\r\n`, transcript.content.encoding).length;
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
    .map((header) => header.toString());
  return headers;
};

export const calculateHeadersRangesExcept = (
  transcript: MessageTranscript,
  exceptHeaders: string[],
) => {
  return pipe(
    transcript.headers,
    getAllHeaders,
    (headers) => filterExceptHeaders(exceptHeaders, headers),
    (filteredHeaders) => {
      return filteredHeaders;
    },
    (filteredHeaders) =>
      calculateHeadersRanges(transcript.message, filteredHeaders),
  );
};

export { calculateHeadersRanges, filterExceptHeaders };
