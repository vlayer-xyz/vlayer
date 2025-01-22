import { describe, expect, test } from "vitest";
import {
  calculateRequestQueryParamsRanges,
  calculateRequestQueryParamsRangesExcept,
} from "./tlsn.request.query.ranges";
import {
  extractUrlQueryParams,
  XAPICallTranscript,
} from "../tlsn.ranges.test.fixtures";

import {
  EncodedString,
  Encoding,
  findUrlInRequest,
  NoGivenParamInUrlError,
} from "../utils";

describe("request url query", () => {
  test("url_query", () => {
    const redactionItem = {
      request: {
        url_query: ["ext"],
      },
    };

    const result = calculateRequestQueryParamsRanges(
      redactionItem.request.url_query,
      XAPICallTranscript.sent.message.content,
      0,
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

    const result = calculateRequestQueryParamsRanges(
      redactionItem.request.url_query,
      XAPICallTranscript.sent.message.content,
      0,
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
    const { url, url_offset } = findUrlInRequest(XAPICallTranscript.sent);
    const result = calculateRequestQueryParamsRanges(
      redactionItem.request.url_query,
      url,
      url_offset,
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

    const result = calculateRequestQueryParamsRanges(
      redactionItem.request.url_query,
      XAPICallTranscript.sent.message.content,
      0,
    );

    expect(result).toEqual([
      {
        start: 110,
        end: 114,
      },
    ]);
  });

  test("no given param in url", () => {
    const redactionItem = {
      request: {
        url_query: ["fregfnergeiogneorgi"],
      },
    };

    const { url, url_offset } = findUrlInRequest(XAPICallTranscript.sent);
    expect(() =>
      calculateRequestQueryParamsRanges(
        redactionItem.request.url_query,
        url,
        url_offset,
      ),
    ).toThrow(new NoGivenParamInUrlError("fregfnergeiogneorgi"));
  });

  test("url_query_except", () => {
    const redactionItem = {
      request: {
        url_query_except: extractUrlQueryParams(
          XAPICallTranscript.sent.message.content.toString(),
        ).filter(
          (param) =>
            ![
              "include_mention_filter",
              "include_ext_dm_nsfw_media_filter",
            ].includes(param),
        ),
      },
    };
    const { url, url_offset } = findUrlInRequest(XAPICallTranscript.sent);
    const result = calculateRequestQueryParamsRangesExcept(
      redactionItem.request.url_query_except,
      url,
      url_offset,
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
    const url = new EncodedString(
      "https://api.example.com/search?param1=value1&param2=value2&param3=value3",
      Encoding.UTF8,
    );
    const url_query_except = ["param1", "param2", "param3"];
    const offset = 4;
    const result = calculateRequestQueryParamsRangesExcept(
      url_query_except,
      url,
      offset,
    );
    expect(result).toEqual([]);
  });

  test("url_query_except with special UTF-8 characters", () => {
    const url = new EncodedString(
      "https://api.example.com/search?name=José&city=SãoPaulo&café=latté",
      Encoding.UTF8,
    );

    const redactionItem = {
      request: {
        url_query_except: ["name", "café"], // Keep these params visible
      },
    };
    const offset = 4;
    const result = calculateRequestQueryParamsRangesExcept(
      redactionItem.request.url_query_except,
      url,
      offset,
    );

    expect(result).toEqual([
      {
        start: 47 + offset,
        end: 56 + offset,
      },
    ]);
  });
});
