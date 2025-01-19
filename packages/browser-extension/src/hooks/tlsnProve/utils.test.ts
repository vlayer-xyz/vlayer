import { describe, expect, test } from "vitest";
import {
  findAllQueryParams,
  findUrlInRequest,
  MessageTranscript,
  parseHttpMessage,
  utf8IndexOf,
  Utf8String,
} from "./utils";
import { InvalidHttpMessageError } from "./tlsn.ranges.error";

describe("utf8IndexOf", () => {
  test("returns 0 when needle is empty", () => {
    const haystack = new Uint8Array([1, 2, 3]);
    const needle = new Uint8Array([]);
    expect(utf8IndexOf(haystack, needle)).toBe(0);
  });

  test("returns -1 when needle is longer than haystack", () => {
    const haystack = new Uint8Array([1, 2]);
    const needle = new Uint8Array([1, 2, 3]);
    expect(utf8IndexOf(haystack, needle)).toBe(-1);
  });

  test("finds needle at start of haystack", () => {
    const haystack = new Uint8Array([1, 2, 3, 4]);
    const needle = new Uint8Array([1, 2]);
    expect(utf8IndexOf(haystack, needle)).toBe(0);
  });

  test("finds needle in middle of haystack", () => {
    const haystack = new Uint8Array([1, 2, 3, 4]);
    const needle = new Uint8Array([2, 3]);
    expect(utf8IndexOf(haystack, needle)).toBe(1);
  });

  test("finds needle at end of haystack", () => {
    const haystack = new Uint8Array([1, 2, 3, 4]);
    const needle = new Uint8Array([3, 4]);
    expect(utf8IndexOf(haystack, needle)).toBe(2);
  });

  test("returns -1 when needle is not found", () => {
    const haystack = new Uint8Array([1, 2, 3, 4]);
    const needle = new Uint8Array([5, 6]);
    expect(utf8IndexOf(haystack, needle)).toBe(-1);
  });

  test("respects from parameter", () => {
    const haystack = new Uint8Array([1, 2, 1, 2]);
    const needle = new Uint8Array([1, 2]);
    expect(utf8IndexOf(haystack, needle, 2)).toBe(2);
  });
});

describe("Utf8String split", () => {
  test("splits string correctly", () => {
    const utf8String = new Utf8String("hello:world");
    expect(utf8String.split(":")).toEqual([
      new Utf8String("hello"),
      new Utf8String("world"),
    ]);
  });
});

describe("Utf8String includes", () => {
  test("returns true when needle is found", () => {
    const utf8String = new Utf8String("hello:world");
    expect(utf8String.includes("hello")).toBe(true);
  });
});

describe("Utf8String equals", () => {
  test("returns true when strings are equal", () => {
    const utf8String = new Utf8String("hello");
    expect(utf8String.equals(new Utf8String("hello"))).toBe(true);
  });
});

describe("Utf8String slice", () => {
  test("slices string correctly", () => {
    const utf8String = new Utf8String("hello:world");
    expect(utf8String.slice(0, 5)).toEqual(new Utf8String("hello"));
  });
});

describe("Utf8String toUtf16String", () => {
  test("returns the string", () => {
    const utf8String = new Utf8String("hello");
    expect(utf8String.toUtf16String()).toEqual("hello");
  });
});

describe("Utf8String nthIndexOf", () => {
  test("returns the index of the nth occurrence of the needle", () => {
    const utf8String = new Utf8String("hello:world:hello:world");
    expect(utf8String.nthIndexOf("hello", 2)).toEqual(12);
  });

  test("returns -1 when the needle is not found", () => {
    const utf8String = new Utf8String("hello:people:hello");
    expect(utf8String.nthIndexOf("world", 2)).toEqual(-1);
  });

  test("returns -1 when the nth is greater than the number of occurrences", () => {
    const utf8String = new Utf8String("hello:world:hello");
    expect(utf8String.nthIndexOf("hello", 3)).toEqual(-1);
  });
});

describe("parseHttpMessage", () => {
  test("parses valid HTTP message correctly", () => {
    const message = [
      "GET /path HTTP/1.1",
      "Host: example.com",
      "Content-Type: application/json",
      "",
      '{"key": "value"}',
    ].join("\r\n");

    const result = parseHttpMessage(message);

    expect(result.info.content.toUtf16String()).toBe("GET /path HTTP/1.1");
    expect(result.headers.content.toUtf16String()).toBe(
      "Host: example.com\r\nContent-Type: application/json",
    );
    expect(result.body.content.toUtf16String()).toBe('{"key": "value"}');

    // Verify ranges
    expect(result.message.range).toEqual({ start: 0, end: message.length });
    expect(result.info.range).toEqual({
      start: 0,
      end: "GET /path HTTP/1.1".length,
    });
    expect(result.headers.range.start).toBeGreaterThan(0);
    expect(result.body.range.start).toBeGreaterThan(result.headers.range.end);
  });

  test("throws error when message cannot be split", () => {
    const message = "Invalid message without separator";
    expect(() => parseHttpMessage(message)).toThrow(InvalidHttpMessageError);
  });

  test("throws error when info line is missing", () => {
    const message = "\r\nContent-Type: text/plain\r\n\r\nbody";
    expect(() => parseHttpMessage(message)).toThrow(InvalidHttpMessageError);
  });

  test("throws error when content-type header is missing", () => {
    const message = [
      "GET /path HTTP/1.1",
      "Host: example.com",
      "",
      "body",
    ].join("\r\n");
    expect(() => parseHttpMessage(message)).toThrow(InvalidHttpMessageError);
  });

  test("converts headers to lowercase", () => {
    const message = [
      "GET /path HTTP/1.1",
      "Host: example.com",
      "CONTENT-TYPE: application/json",
      "",
      "body",
    ].join("\r\n");

    const result = parseHttpMessage(message);
    expect(result.headers.content.toUtf16String()).toBe(
      "Host: example.com\r\nCONTENT-TYPE: application/json",
    );
  });
});

describe("findUrlInRequest", () => {
  test("extracts URL and offset from request transcript", () => {
    const transcript = {
      message: {
        content: new Utf8String("GET /path?param=value HTTP/1.1"),
      },
    } as MessageTranscript;

    const result = findUrlInRequest(transcript);

    expect(result.url.toUtf16String()).toBe("/path?param=value");
    expect(result.url_offset).toBe(4);
  });
});

describe("findAllQueryParams", () => {
  test("extracts query parameters from URL", () => {
    const path = "https://example.com/path?param1=value1&param2=value2";

    const result = findAllQueryParams(path);

    expect(result).toEqual(["param1", "param2"]);
  });

  test("returns empty array for URL without query params", () => {
    const path = "https://example.com/path";

    const result = findAllQueryParams(path);

    expect(result).toEqual([]);
  });

  test("handles URLs with repeated parameters", () => {
    const path = "https://example.com/path?param1=value1&param1=value2";

    const result = findAllQueryParams(path);

    expect(result).toEqual(["param1"]);
  });
});
