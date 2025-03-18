// Test for calculateRequestSize
import { describe, expect, test } from "vitest";
import { calculateRequestSize } from "./calculateRequestSize";
import { CRLF_SIZE } from "./constants";
import { constraints } from "./constraints";
import { getRequestLineSize } from "./requestLine/requestLine";
import { getBodySize } from "./body/body";
import { getHeadersSize } from "./headers/headers";
import { RequestSizeParams } from "./types";

describe("calculateRequestSize", () => {
  const testCases = [
    {
      description: "GET request with custom header and body",
      url: "http://example.com",
      method: "GET",
      headers: { "Custom-Header": "CustomValue" },
      body: JSON.stringify({ key: "value" }),
    },
    {
      description: "POST request with multiple headers and empty body",
      url: "http://example.com/api",
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: "Bearer token",
      },
      body: JSON.stringify({}),
    },
    {
      description: "PUT request with no headers and simple body",
      url: "http://example.com/update",
      method: "PUT",
      headers: {},
      body: JSON.stringify({ update: "data" }),
    },
    {
      description: "DELETE request with headers and no body",
      url: "http://example.com/delete",
      method: "DELETE",
      headers: { Authorization: "Bearer token" },
      body: undefined,
    },
  ] as (RequestSizeParams & { description: string })[];

  testCases.forEach(({ description, url, method, headers, body }) => {
    test(`calculates the correct request size for ${description}`, () => {
      const expectedRequestLineSize = getRequestLineSize(
        url,
        method,
        constraints,
      );
      const expectedBodySize = getBodySize(body);
      const expectedHeadersSize = getHeadersSize(
        url,
        headers,
        body,
        constraints,
      );

      const totalExpectedSize =
        expectedRequestLineSize +
        expectedHeadersSize +
        CRLF_SIZE +
        expectedBodySize;

      const calculatedSize = calculateRequestSize({
        url,
        method,
        headers,
        body,
      });

      expect(calculatedSize).toBe(totalExpectedSize);
    });
  });
});
