import { describe, expect, test } from "vitest";
import { makeHeaders, toHeadersString } from "./headers";
import { CRLF, RequestLineMode } from "../constants";

describe("Header Utilities", () => {
  const testConstraints = {
    requestLineMode: RequestLineMode.FULL_PATH,
    protocol: "HTTP/1.1",
    defaultHeaders: {
      Host: ({ url }: { url: string }) => new URL(url).hostname,
      Connection: () => "close",
      "Content-Length": ({ body }: { body?: string }) =>
        body ? new TextEncoder().encode(body).length.toString() : "0",
    },
  };

  const testUrl = "http://example.com";
  const testHeaders = { "Custom-Header": "CustomValue" };
  const testBody = JSON.stringify({ key: "value" });

  test("generate correct headers using makeHeaders", () => {
    const generatedHeaders = makeHeaders(
      testUrl,
      testHeaders,
      testBody,
      testConstraints,
    );

    expect(generatedHeaders["Host"]).toBe("example.com");
    expect(generatedHeaders["Connection"]).toBe("close");
    expect(generatedHeaders["Content-Length"]).toBe("15");
    expect(generatedHeaders["Custom-Header"]).toBe("CustomValue");
    expect(Object.keys(generatedHeaders).length).toEqual(
      Object.keys(testConstraints.defaultHeaders).length +
        Object.keys(testHeaders).length,
    );
  });

  test("convert headers to string correctly using toHeadersString", () => {
    const generatedHeaders = makeHeaders(
      testUrl,
      testHeaders,
      testBody,
      testConstraints,
    );
    const headersString = toHeadersString(generatedHeaders);

    expect(headersString).toContain(`Host: example.com${CRLF}`);
    expect(headersString).toContain(`Connection: close${CRLF}`);
    expect(headersString).toContain(`Content-Length: 15${CRLF}`);
    expect(headersString).toContain(`Custom-Header: CustomValue${CRLF}`);
    expect(headersString.length).toBe(
      `Host: example.com${CRLF}Connection: close${CRLF}Content-Length: 15${CRLF}Custom-Header: CustomValue${CRLF}`
        .length,
    );
  });

  test("headers overwrite default headers", () => {
    const customHeaders = {
      Host: "customhost.com",
      Connection: "keep-alive",
    };

    const generatedHeaders = makeHeaders(
      testUrl,
      customHeaders,
      testBody,
      testConstraints,
    );

    expect(generatedHeaders["Host"]).toBe("customhost.com");
    expect(generatedHeaders["Connection"]).toBe("keep-alive");
  });
});
