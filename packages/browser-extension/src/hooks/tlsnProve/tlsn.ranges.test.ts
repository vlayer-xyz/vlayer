import { describe, expect, test } from "vitest";
import { calculateResponseRanges, calculateRequestRanges } from "./tlsn.ranges";
import {
  RedactResponseHeaders,
  RedactResponseHeadersExcept,
} from "src/web-proof-commons/types/message";
import {
  fixtureTranscript,
  fixtureAllResponseHeaders,
  fixtureAllRequestHeaders,
  fixtureAllUrlQueries
} from "./tlsn.ranges.test.fixtures";

describe("response headers", () => {
  test("single header", () => {
    const redactionItem: RedactResponseHeaders = {
      response: {
        headers: ["pragma"],
      },
    };

    const result = calculateResponseRanges(
      redactionItem,
      fixtureTranscript.recv,
      fixtureTranscript.ranges.recv,
    );

    expect(result).toEqual([
      {
        start: 79,
        end: 88,
      },
    ]);
  });

  test("multiple headers", () => {
    const redactionItem: RedactResponseHeaders = {
      response: {
        headers: ["status", "expires"],
      },
    };

    const result = calculateResponseRanges(
      redactionItem,
      fixtureTranscript.recv,
      fixtureTranscript.ranges.recv,
    );

    expect(result).toEqual([
      {
        start: 112,
        end: 119,
      },
      {
        start: 129,
        end: 159,
      },
    ]);
  });

  test("headers_except", () => {
    const redactionItem: RedactResponseHeadersExcept = {
      response: {
        headers_except: fixtureAllResponseHeaders.filter(
          (header) => !["content-type", "x-transaction-id"].includes(header),
        ),
      },
    };

    const result = calculateResponseRanges(
      redactionItem,
      fixtureTranscript.recv,
      fixtureTranscript.ranges.recv,
    );

    expect(result).toEqual([
      {
        start: 174,
        end: 205,
      },
      {
        start: 476,
        end: 493,
      },
    ]);
  });
});

describe("request headers", () => {
  test("single header", () => {
    const redactionItem = {
      request: {
        headers: ["accept-encoding"],
      },
    };

    const result = calculateRequestRanges(
      redactionItem,
      fixtureTranscript.sent,
      fixtureTranscript.ranges.sent,
    );

    expect(result).toEqual([{
      start: 489,
      end: 498,
    }]);
  });

  test("multiple headers", () => {
    const redactionItem = {
      request: {
        headers: ["authorization", "cookie"],
      },
    };

    const result = calculateRequestRanges(
      redactionItem,
      fixtureTranscript.sent,
      fixtureTranscript.ranges.sent,
    );

    expect(result).toEqual([
      {
        start: 624,
        end: 736,
      },
      {
        start: 864,
        end: 1481,
      },
    ]);
  });

  test("headers_except", () => {
    const redactionItem = {
      request: {
        headers_except: fixtureAllRequestHeaders.filter(
          (header) => !["x-client-transaction-id", "connection"].includes(header),
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
        start: 376,
        end: 471,
      },
      {
        start: 1699,
        end: 1705,
      },
    ]);
  });

  test("headers_except with all headers", () => {
    const redactionItem = {
      request: {
        headers_except: fixtureAllRequestHeaders,
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
  })

  test("url_query_except", () => {
    const redactionItem = {
      request: {
        url_query_except: fixtureAllUrlQueries.filter(
          (query) => !["include_mention_filter", "include_ext_dm_nsfw_media_filter"].includes(query),
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
