import { expect, describe, test } from "vitest";
import { parseHttpMessage } from "./parseMessage";
import { InvalidHttpMessageError } from "./error";

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
