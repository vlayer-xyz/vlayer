import {
  RedactResponseHeaders,
  RedactResponseHeadersExcept,
} from "src/web-proof-commons/types/message";
import { describe, expect, test } from "vitest";
import {
  fixtureAllResponseHeaders,
  fixtureTranscript,
} from "./tlsn.ranges.test.fixtures";
import { calculateResponseRanges } from "./tlsn.response.ranges";

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

  test("headers case insensitive", () => {
    const redactionItem: RedactResponseHeaders = {
      response: {
        headers: ["Pragma", "sTaTuS"],
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
      {
        start: 112,
        end: 119,
      },
    ]);
  });

  test("not existing header", () => {
    const redactionItem: RedactResponseHeaders = {
      response: {
        headers: ["not-existing-header"],
      },
    };

    expect(() =>
      calculateResponseRanges(
        redactionItem,
        fixtureTranscript.recv,
        fixtureTranscript.ranges.recv,
      ),
    ).toThrowError("Header not-existing-header not found");
  });
});
