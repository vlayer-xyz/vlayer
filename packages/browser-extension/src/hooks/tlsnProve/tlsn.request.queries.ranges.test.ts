import { describe, expect, test } from "vitest";
import { calculateRequestRanges } from "./tlsn.request.ranges";
import {
  fixtureAllUrlQueries,
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
        start: 259,
        end: 277,
      },
    ]);
  });

  test("url_query with multiple queries", () => {
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
        start: 259,
        end: 277,
      },
      {
        start: 115,
        end: 142,
      },
    ]);
  });

  test("url_query with the last query", () => {
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
        start: 304,
        end: 341,
      },
    ]);
  });

  test("url_query_except", () => {
    const redactionItem = {
      request: {
        url_query_except: fixtureAllUrlQueries.filter(
          (query) =>
            ![
              "include_mention_filter",
              "include_ext_dm_nsfw_media_filter",
            ].includes(query),
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
        start: 115,
        end: 142,
      },
      {
        start: 304,
        end: 341,
      },
    ]);
  });

  test("url_query_except with all queries", () => {
    const redactionItem = {
      request: {
        url_query_except: fixtureAllUrlQueries,
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
