import {
  InvalidEncodingError,
  InvalidHttpMessageError,
} from "hooks/tlsnProve/error";
import { EncodedString } from "./EncodedString";
import { Encoding } from "./Encoding";

const bodyHeaderDelimiter = "\r\n\r\n";
const newLine = "\r\n";
const keyValueDelimiter = ": ";

export function parseHttpMessage(message: string) {
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

  if (!contentType) {
    throw new InvalidHttpMessageError("No content-type header found");
  }

  const encoding = contentType.split("charset=")[1].trim();

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
