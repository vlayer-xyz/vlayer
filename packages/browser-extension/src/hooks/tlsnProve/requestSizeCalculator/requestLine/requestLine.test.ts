import { describe, expect, test } from "vitest";
import { getRequestLineSize } from "./requestLine";
import { CRLF, RequestLineMode } from "../constants";
import { makeRequestLine } from "./requestLine";

describe("Request Line Utilities", () => {
  const testConstraints = {
    requestLineMode: RequestLineMode.FULL_PATH,
    protocol: "HTTP/1.1",
  };

  const testUrl = "http://example.com";
  const testMethod = "GET";

  test("generate correct request line using makeRequestLine", () => {
    const requestLine = makeRequestLine(testUrl, testMethod, testConstraints);
    expect(requestLine).toBe(`GET http://example.com/ HTTP/1.1${CRLF}`);
  });

  test("calculate correct request line size using getRequestLineSize", () => {
    const requestLineSize = getRequestLineSize(
      testUrl,
      testMethod,
      testConstraints,
    );
    expect(requestLineSize).toBe(
      `GET http://example.com/ HTTP/1.1${CRLF}`.length,
    );
  });
});
