import { InvalidHttpMessageError } from "./tlsn.ranges.error";
import { CommitData } from "tlsn-js/src/types";

export type Transcript = {
  sent: string;
  recv: string;
};

export type MessagePartTranscript = {
  content: Utf8String;
  range: CommitData;
};

export type MessageTranscript = {
  message: MessagePartTranscript;
  info: MessagePartTranscript;
  headers: MessagePartTranscript;
  body: MessagePartTranscript;
};

export type Utf8Transcript = {
  sent: MessageTranscript;
  recv: MessageTranscript;
};

export function utf8IndexOf(
  haystack: Uint8Array,
  needle: Uint8Array,
  from: number = 0,
) {
  const haystackLen = haystack.length;
  const needleLen = needle.length;

  if (needleLen === 0) {
    return 0;
  }
  if (needleLen > haystackLen) {
    return -1;
  }

  return (
    Array.from(
      { length: haystackLen - needleLen + 1 },
      (_, i) => i + from,
    ).find((i) => needle.every((byte, j) => haystack[i + j] === byte)) ?? -1
  );
}

export class Utf8String {
  private value: Uint8Array;
  private utf16String: string;

  constructor(stringValue: string) {
    this.value = new TextEncoder().encode(stringValue);
    this.utf16String = stringValue;
  }

  indexOf(needle: Utf8String | string, from: number = 0): number {
    const needleValue =
      needle instanceof Utf8String
        ? needle.value
        : new TextEncoder().encode(needle);
    return utf8IndexOf(this.value, needleValue, from);
  }
  nthIndexOf(needle: string, n: number, from: number = 0): number {
    let count = 0;
    while (count < n) {
      count++;

      const index = this.indexOf(needle, from);
      if (index === -1) {
        return -1;
      }
      from = index + 1;
    }
    return from - 1;
  }

  get length() {
    return this.value.length;
  }

  split(separator: string | Utf8String): Utf8String[] {
    return this.utf16String
      .split(
        separator instanceof Utf8String ? separator.utf16String : separator,
      )
      .map((str) => new Utf8String(str));
  }

  includes(needle: string | Utf8String): boolean {
    return this.utf16String.includes(
      needle instanceof Utf8String ? needle.utf16String : needle,
    );
  }

  equals(other: Utf8String) {
    return this.utf16String === other.utf16String;
  }

  toUtf16String() {
    return this.utf16String;
  }

  slice(start: number, end: number) {
    const slicedArray = this.value.slice(start, end);
    const decodedString = new TextDecoder().decode(slicedArray);
    return new Utf8String(decodedString);
  }
}

export const toUtf8Transcript = (transcript: Transcript): Utf8Transcript => {
  const parsedRecvMessage = parseHttpMessage(transcript.recv);
  const parsedSentMessage = parseHttpMessage(transcript.sent);
  return {
    recv: parsedRecvMessage,
    sent: parsedSentMessage,
  };
};

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

export function findUrlInRequest(transcript: MessageTranscript): {
  url: Utf8String;
  url_offset: number;
} {
  const url = transcript.message.content.split(" ")[1];
  return {
    url,
    url_offset: transcript.message.content.indexOf(url),
  };
}

export function findAllQueryParams(path: string): string[] {
  return [...new Set(Array.from(new URL(path).searchParams.keys()))];
}
