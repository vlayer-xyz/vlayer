import { describe, expect, test } from "vitest";
import { calculateRequestRanges } from "./tlsn.request.ranges";
import {
  fixtureAllUrlQueryParams,
  fixtureTranscript,
} from "./tlsn.ranges.test.fixtures";

describe("request url query", () => {
  test("url_query", () => {
    const redactionItem = {
      request: {
        url_query: ["ext"],
      },
    };

    const result = calculateRequestRanges(
      redactionItem,
      fixtureTranscript.sent,
      fixtureTranscript.ranges.sent,
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

    const result = calculateRequestRanges(
      redactionItem,
      fixtureTranscript.sent,
      fixtureTranscript.ranges.sent,
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

    const result = calculateRequestRanges(
      redactionItem,
      fixtureTranscript.sent,
      fixtureTranscript.ranges.sent,
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

    const result = calculateRequestRanges(
      redactionItem,
      fixtureTranscript.sent,
      fixtureTranscript.ranges.sent,
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
        url_query_except: fixtureAllUrlQueryParams.filter(
          (param) =>
            ![
              "include_mention_filter",
              "include_ext_dm_nsfw_media_filter",
            ].includes(param),
        ),
      },
    };

    const result = calculateRequestRanges(
      redactionItem,
      fixtureTranscript.sent,
      fixtureTranscript.ranges.sent,
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
        url_query_except: fixtureAllUrlQueryParams,
      },
    };

    const result = calculateRequestRanges(
      redactionItem,
      fixtureTranscript.sent,
      fixtureTranscript.ranges.sent,
    );

    expect(result).toEqual([]);
  });
});
