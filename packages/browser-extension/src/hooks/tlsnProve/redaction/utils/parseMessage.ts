import { InvalidHttpMessageError } from "./error";
import { Transcript, Utf8Transcript } from "./types";
import { Utf8String } from "./utf8String";

export function parseHttpMessage(message: string) {
  const [headerPart, bodyPart] = message.split("\r\n\r\n");
  if (typeof headerPart === "undefined" || typeof bodyPart === "undefined") {
    throw new InvalidHttpMessageError(
      "Cannot split message into header and body",
    );
  }

  const headerLines = headerPart.split("\r\n");
  const info = headerLines.shift();
  if (!info) {
    throw new InvalidHttpMessageError("No info line found");
  }
  const headers: Record<string, string> = {};

  headerLines.forEach((line) => {
    const [key, value] = line.split(": ");
    headers[key.toLowerCase()] = value;
  });
  const contentType = headers["content-type"];

  if (!contentType) {
    throw new InvalidHttpMessageError("No content-type header found");
  }

  const utf8Info = new Utf8String(info);
  const utf8Message = new Utf8String(message);
  const utf8Headers = new Utf8String(headerLines.join("\r\n"));
  const utf8Body = new Utf8String(bodyPart);

  return {
    message: {
      content: utf8Message,
      range: {
        start: 0,
        end: utf8Message.length,
      },
    },
    info: {
      content: utf8Info,
      range: {
        start: 0,
        end: utf8Info.length,
      },
    },
    headers: {
      content: utf8Headers,
      range: {
        start: utf8Message.indexOf(utf8Headers),
        end: utf8Message.indexOf(utf8Headers) + utf8Headers.length,
      },
    },
    body: {
      content: utf8Body,
      range: {
        start: utf8Message.indexOf(utf8Body),
        end: utf8Message.indexOf(utf8Body) + utf8Body.length,
      },
    },
  };
}

export const toUtf8Transcript = (transcript: Transcript): Utf8Transcript => {
  const parsedRecvMessage = parseHttpMessage(transcript.recv);
  const parsedSentMessage = parseHttpMessage(transcript.sent);
  return {
    recv: parsedRecvMessage,
    sent: parsedSentMessage,
  };
};
