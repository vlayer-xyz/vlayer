import { describe, expect, test } from "vitest";
import {
  fixtureAllRequestHeaders,
  fixtureTranscript,
} from "./tlsn.ranges.test.fixtures";
import { calculateRequestRanges } from "./tlsn.request.ranges";

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

    expect(result).toEqual([
      {
        start: 489,
        end: 498,
      },
    ]);
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
          (header) =>
            !["x-client-transaction-id", "connection"].includes(header),
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

  test("headers case insensitivity", () => {
    const redactionItem = {
      request: {
        headers: ["Accept-Encoding", "CoOkIe"],
      },
    };

    const result = calculateRequestRanges(
      redactionItem,
      fixtureTranscript.sent,
      fixtureTranscript.ranges.sent,
    );

    expect(result).toEqual([
      {
        start: 489,
        end: 498,
      },
      {
        start: 864,
        end: 1481,
      },
    ]);
  });

  test("not existing header", () => {
    const redactionItem = {
      request: {
        headers: ["not-existing-header"],
      },
    };

    expect(() =>
      calculateRequestRanges(
        redactionItem,
        fixtureTranscript.sent,
        fixtureTranscript.ranges.sent,
      ),
    ).toThrowError("Header not-existing-header not found");
  });
});
