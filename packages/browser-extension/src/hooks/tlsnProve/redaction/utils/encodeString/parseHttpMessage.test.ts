import { expect, test, describe } from "vitest";
import { parseHttpMessage } from "./parseHttpMessage";
import { InvalidEncodingError, InvalidHttpMessageError } from "../error";
import { Encoding } from "./Encoding";

describe("parseHttpMessage", () => {
  test("should parse a valid HTTP message correctly", () => {
    const validMessage = `POST /path HTTP/1.1\r\nContent-Type: application/json; charset=utf-8\r\n\r\n{"key": "value"}`;
    const result = parseHttpMessage(validMessage);
    expect(result.info.content.toString()).toEqual("POST /path HTTP/1.1");
    expect(result.headers.content.toString()).toBe(
      "Content-Type: application/json; charset=utf-8",
    );
    expect(result.body.content.toString()).toBe('{"key": "value"}');
    expect(result.message.content.toString()).toBe(validMessage);
  });

  test("should parse a valid HTTP message with UTF-16 and special chars correctly", () => {
    const validMessage = `POST /héllo HTTP/1.1\r\nContent-Type: application/json; charset=utf-16\r\n\r\n{"key": "👋 världen"}`;
    const result = parseHttpMessage(validMessage);
    expect(result.info.content.toString()).toEqual("POST /héllo HTTP/1.1");
    expect(result.headers.content.toString()).toBe(
      "Content-Type: application/json; charset=utf-16",
    );
    expect(result.body.content.toString()).toBe('{"key": "👋 världen"}');
    expect(result.message.content.toString()).toBe(validMessage);
  });

  test("throws on invalid encoding", () => {
    const invalidMessage = `POST /path HTTP/1.1\r\nContent-Type: application/json; charset=invalid-encoding\r\n\r\n{"key": "value"}`;
    expect(() => parseHttpMessage(invalidMessage)).toThrow(
      new InvalidEncodingError("invalid-encoding"),
    );
  });

  test("should throw error for invalid HTTP message format", () => {
    const invalidMessage = "Invalid message without proper structure";
    expect(() => parseHttpMessage(invalidMessage)).toThrow(
      InvalidHttpMessageError,
    );
  });

  test("should throw error when content-type header is missing", () => {
    const messageWithoutContentType = `POST /path HTTP/1.1\r\nOther-Header: value\r\n\r\nBody content`;
    expect(() => parseHttpMessage(messageWithoutContentType)).toThrow(
      new InvalidHttpMessageError("No content-type header found"),
    );
  });

  test("should calculate correct ranges with UTF-8 encoding", () => {
    const message = `POST /path HTTP/1.1\r\nContent-Type: text/plain; charset=utf-8\r\nX-Custom: test\r\n\r\nHello World 👋`;
    const result = parseHttpMessage(message);

    expect(result.message.range).toEqual({
      start: 0,
      end: [...new TextEncoder().encode(message)].length,
    });

    expect(result.info.range).toEqual({
      start: 0,
      end: "POST /path HTTP/1.1".length,
    });

    const headersContent =
      "Content-Type: text/plain; charset=utf-8\r\nX-Custom: test";
    expect(result.headers.range).toEqual({
      start: message.indexOf(headersContent),
      end:
        message.indexOf(headersContent) +
        [...new TextEncoder().encode(headersContent)].length,
    });

    const bodyContent = "Hello World 👋";
    expect(result.body.range).toEqual({
      start: message.indexOf(bodyContent),
      end:
        message.indexOf(bodyContent) +
        [...new TextEncoder().encode(bodyContent)].length,
    });
  });

  test("should calculate correct ranges with UTF-16 encoding", () => {
    const message = `POST /path HTTP/1.1\r\nContent-Type: text/plain; charset=utf-16\r\nX-Custom: test\r\n\r\nHéllo World 🌍`;
    const result = parseHttpMessage(message);

    expect(result.message.range).toEqual({
      start: 0,
      end: message.length * 2,
    });

    expect(result.info.range).toEqual({
      start: 0,
      end: "POST /path HTTP/1.1".length * 2,
    });

    const headersContent =
      "Content-Type: text/plain; charset=utf-16\r\nX-Custom: test";
    expect(result.headers.range).toEqual({
      start: message.indexOf(headersContent) * 2,
      end: message.indexOf(headersContent) * 2 + headersContent.length * 2,
    });

    const bodyContent = "Héllo World 🌍";
    expect(result.body.range).toEqual({
      start: message.indexOf(bodyContent) * 2,
      end: message.indexOf(bodyContent) * 2 + bodyContent.length * 2,
    });
  });

  test("should calculate correct ranges with UTF-8 special characters", () => {
    const message = `POST /path HTTP/1.1\r\nContent-Type: text/plain; charset=utf-8\r\nX-Custom: test\r\n\r\nHéllo Wörld 🌍`;
    const result = parseHttpMessage(message);

    expect(result.message.range).toEqual({
      start: 0,
      end: [...new TextEncoder().encode(message)].length,
    });

    expect(result.info.range).toEqual({
      start: 0,
      end: "POST /path HTTP/1.1".length,
    });

    const headersContent =
      "Content-Type: text/plain; charset=utf-8\r\nX-Custom: test";
    expect(result.headers.range).toEqual({
      start: message.indexOf(headersContent),
      end: message.indexOf(headersContent) + headersContent.length,
    });

    const bodyContent = "Héllo Wörld 🌍";
    expect(result.body.range).toEqual({
      start: message.indexOf(bodyContent),
      end:
        message.indexOf(bodyContent) +
        [...new TextEncoder().encode(bodyContent)].length,
    });
  });
  test("should not throw error when content-type header is missing and enforceContentType is false", () => {
    const messageWithoutContentType = `POST /path HTTP/1.1\r\nOther-Header: value\r\n\r\nBody content`;
    expect(() =>
      parseHttpMessage(messageWithoutContentType, {
        enforceContentType: false,
        defaultEncoding: Encoding.UTF8,
      }),
    ).not.toThrow();
  });

  test("should use default encoding when content-type header is missing and enforceContentType is false", () => {
    const messageWithoutContentType = `POST /path HTTP/1.1\r\nOther-Header: value\r\n\r\nBody content`;
    const transcript = parseHttpMessage(messageWithoutContentType, {
      enforceContentType: false,
      defaultEncoding: Encoding.UTF16,
    });
    expect(transcript.encoding).toEqual(Encoding.UTF16);
  });
  test("should use default encoding when content-type header contains no charset", () => {
    const message = `POST /path HTTP/1.1\r\nContent-Type: application/json\r\n\r\nBody content`;
    const transcript = parseHttpMessage(message, {
      enforceContentType: false,
      defaultEncoding: Encoding.UTF16,
    });
    expect(transcript.encoding).toEqual(Encoding.UTF16);
  });
});
