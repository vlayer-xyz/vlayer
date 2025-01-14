import { describe, expect, test } from "vitest";
import { calculateResponseRanges } from "./tlsn.ranges";
import {
  RedactResponseHeaders,
  RedactResponseHeadersExcept,
} from "src/web-proof-commons/types/message";
import {
  fixtureTranscript,
  fixtureAllResponseHeaders,
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
