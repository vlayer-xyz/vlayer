import { expect, describe, test } from "vitest";
import { getQueryParams, getRequestUrl } from "./getRequestUrl";
import type { MessageTranscript } from "./types";
import { Utf8String } from "./utf8String";

describe("getRequestUrl", () => {
  test("extracts URL and offset from request transcript", () => {
    const transcript = {
      message: {
        content: new Utf8String("GET /path?param=value HTTP/1.1"),
      },
    } as MessageTranscript;

    const result = getRequestUrl(transcript);

    expect(result.url.toUtf16String()).toBe("/path?param=value");
    expect(result.url_offset).toBe(4);
  });
});

describe("getQueryParams", () => {
  test("extracts query parameters from URL", () => {
    const path = "https://example.com/path?param1=value1&param2=value2";

    const result = getQueryParams(path);

    expect(result).toEqual(["param1", "param2"]);
  });

  test("returns empty array for URL without query params", () => {
    const path = "https://example.com/path";

    const result = getQueryParams(path);

    expect(result).toEqual([]);
  });

  test("handles URLs with repeated parameters", () => {
    const path = "https://example.com/path?param1=value1&param1=value2";

    const result = getQueryParams(path);

    expect(result).toEqual(["param1"]);
  });
});
