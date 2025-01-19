import { describe, expect, test } from "vitest";
import { calculateRequestRedactionRanges } from "./tlsn.request.ranges";
import {
  extractUrlQueryParams,
  XAPICallTranscript,
} from "./tlsn.ranges.test.fixtures";
import { MessageTranscript, Utf8String } from "./utils";

describe("request url query", () => {
  test("url_query", () => {
    const redactionItem = {
      request: {
        url_query: ["ext"],
      },
    };

    const result = calculateRequestRedactionRanges(
      redactionItem,
      XAPICallTranscript.sent,
    );

    expect(result).toEqual([
      {
        start: 263,
        end: 277,
      },
    ]);
  });

  test("url_query with multiple parameters", () => {
    const redactionItem = {
      request: {
        url_query: ["ext", "include_mention_filter"],
      },
    };

    const result = calculateRequestRedactionRanges(
      redactionItem,
      XAPICallTranscript.sent,
    );

    expect(result).toEqual([
      {
        start: 263,
        end: 277,
      },
      {
        start: 138,
        end: 142,
      },
    ]);
  });

  test("url_query with the last parameter", () => {
    const redactionItem = {
      request: {
        url_query: ["include_ext_dm_nsfw_media_filter"],
      },
    };

    const result = calculateRequestRedactionRanges(
      redactionItem,
      XAPICallTranscript.sent,
    );

    expect(result).toEqual([
      {
        start: 337,
        end: 341,
      },
    ]);
  });

  test("url_query with the first parameter", () => {
    const redactionItem = {
      request: {
        url_query: [
          "include_ext_sharing_audiospaces_listening_data_with_followers",
        ],
      },
    };

    const result = calculateRequestRedactionRanges(
      redactionItem,
      XAPICallTranscript.sent,
    );

    expect(result).toEqual([
      {
        start: 110,
        end: 114,
      },
    ]);
  });

  test("url_query_except", () => {
    const redactionItem = {
      request: {
        url_query_except: extractUrlQueryParams(
          XAPICallTranscript.sent.message.content.toUtf16String(),
        ).filter(
          (param) =>
            ![
              "include_mention_filter",
              "include_ext_dm_nsfw_media_filter",
            ].includes(param),
        ),
      },
    };

    const result = calculateRequestRedactionRanges(
      redactionItem,
      XAPICallTranscript.sent,
    );

    expect(result).toEqual([
      {
        start: 138,
        end: 142,
      },
      {
        start: 337,
        end: 341,
      },
    ]);
  });

  test("url_query_except with all query parameters", () => {
    const redactionItem = {
      request: {
        url_query_except: extractUrlQueryParams(
          XAPICallTranscript.sent.message.content.toUtf16String(),
        ),
      },
    };

    const result = calculateRequestRedactionRanges(
      redactionItem,
      XAPICallTranscript.sent,
    );

    expect(result).toEqual([]);
  });

  test.todo("url_query_param with space inside", () => {
    const fakeTranscript = {
      message: {
        content: new Utf8String(
          "GET https://api.example.com/search?name=José&city=São Paulo&café=latté HTTP/1.1\r\n" +
            "Host: example.com\r\n\r\n",
        ),
      },
    } as MessageTranscript;

    const redactionItem = {
      request: {
        url_query_except: ["name", "café"],
      },
    };

    const result = calculateRequestRedactionRanges(
      redactionItem,
      fakeTranscript,
    );

    expect(result).toEqual([
      {
        start: 51,
        end: 61,
      },
    ]);
  });
  test("url_query_except with special UTF-8 characters", () => {
    const fakeTranscript = {
      message: {
        content: new Utf8String(
          "GET https://api.example.com/search?name=José&city=SãoPaulo&café=latté HTTP/1.1\r\n" +
            "Host: example.com\r\n\r\n",
        ),
      },
    } as MessageTranscript;

    const redactionItem = {
      request: {
        url_query_except: ["name", "café"], // Keep these params visible
      },
    };

    const result = calculateRequestRedactionRanges(
      redactionItem,
      fakeTranscript,
    );

    expect(result).toEqual([
      {
        start: 51,
        end: 60,
      },
    ]);
  });
});
