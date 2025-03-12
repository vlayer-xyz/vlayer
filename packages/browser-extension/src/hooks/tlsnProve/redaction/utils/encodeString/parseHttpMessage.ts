import { MessageTranscript } from "../../types";
import { InvalidEncodingError, InvalidHttpMessageError } from "../error";
import { EncodedString } from "./EncodedString";
import { Encoding } from "./Encoding";

const bodyHeaderDelimiter = "\r\n\r\n";
const newLine = "\r\n";
const keyValueDelimiter = ": ";

export function parseHttpMessage(
  message: string,
  {
    enforceContentType,
    defaultEncoding,
  }: { enforceContentType: boolean; defaultEncoding: Encoding } = {
    enforceContentType: true,
    defaultEncoding: Encoding.UTF8,
  },
): MessageTranscript {
  const [headerPart, bodyPart] = message.split(bodyHeaderDelimiter);
  if (typeof headerPart === "undefined" || typeof bodyPart === "undefined") {
    throw new InvalidHttpMessageError(
      "Cannot split message into header and body",
    );
  }

  const headerLines = headerPart.split(newLine);
  const [info, ...remainingHeaders] = headerLines;
  if (!info) {
    throw new InvalidHttpMessageError("No info line found");
  }

  const contentTypeHeader = remainingHeaders.find((line) =>
    line.toLowerCase().startsWith("content-type" + keyValueDelimiter),
  );

  const contentType = contentTypeHeader?.split(keyValueDelimiter)[1];

  if (!contentType && enforceContentType) {
    throw new InvalidHttpMessageError("No content-type header found");
  }

  //if we do not enforce content type, we use the default encoding
  const encoding =
    contentType?.split("charset=")[1]?.trim().toLowerCase() ?? defaultEncoding;

  if (!validateEncoding(encoding)) {
    throw new InvalidEncodingError(encoding);
  }

  const infoEncoded = new EncodedString(info, encoding);
  const messageEncoded = new EncodedString(message, encoding);
  const headersEncoded = new EncodedString(
    remainingHeaders.join(newLine),
    encoding,
  );
  const bodyEncoded = new EncodedString(bodyPart, encoding);

  return {
    encoding,
    message: {
      content: messageEncoded,
      range: {
        start: 0,
        end: messageEncoded.length,
      },
    },
    info: {
      content: infoEncoded,
      range: {
        start: 0,
        end: infoEncoded.length,
      },
    },
    headers: {
      content: headersEncoded,
      range: {
        start: messageEncoded.indexOf(headersEncoded),
        end: messageEncoded.indexOf(headersEncoded) + headersEncoded.length,
      },
    },
    body: {
      content: bodyEncoded,
      range: {
        start: messageEncoded.indexOf(bodyEncoded),
        end: messageEncoded.indexOf(bodyEncoded) + bodyEncoded.length,
      },
    },
  };
}

const validateEncoding = (encoding: string): encoding is Encoding => {
  return Object.values(Encoding).includes(encoding as Encoding);
};

export const parseTlsnTranscript = ({
  recv,
  sent,
}: {
  recv: string;
  sent: string;
}) => {
  return {
    recv: parseHttpMessage(recv, {
      enforceContentType: false,
      defaultEncoding: Encoding.UTF8,
    }),
    sent: parseHttpMessage(sent, {
      enforceContentType: false,
      defaultEncoding: Encoding.UTF8,
    }),
  };
};
