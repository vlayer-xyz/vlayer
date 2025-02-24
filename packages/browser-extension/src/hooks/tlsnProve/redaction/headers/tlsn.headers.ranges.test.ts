import { RedactResponseHeaders } from "src/web-proof-commons";
import { describe, expect, test } from "vitest";
import {
  getHeaderRange,
  XAPICallTranscript,
  TranscriptWithDoubleHeaders,
  allRequestHeadersRedactedRanges,
  allResponseHeadersRedactedRanges,
} from "../tlsn.ranges.test.fixtures";
import {
  calculateHeadersRanges,
  calculateHeadersRangesExcept,
} from "./tlsn.headers.ranges";
import { HeaderNotFoundError } from "../utils";

import { extractHeaders } from "../tlsn.ranges.test.fixtures";

const fixtureAllRequestHeaders = extractHeaders(
  XAPICallTranscript.sent.message.content.toString(),
);

const fixtureAllResponseHeaders = extractHeaders(
  XAPICallTranscript.recv.message.content.toString(),
);

describe("headers redaction", () => {
  describe("response headers", () => {
    test("single response header", () => {
      const redactionItem: RedactResponseHeaders = {
        response: {
          headers: ["date"],
        },
      };

      const result = calculateHeadersRanges(
        XAPICallTranscript.recv.message,
        redactionItem.response.headers,
      );

      expect(result).toEqual([getHeaderRange(XAPICallTranscript.recv, "date")]);
    });

    test("multiple response headers", () => {
      const redactionItem: RedactResponseHeaders = {
        response: {
          headers: ["status", "expires"],
        },
      };

      const result = calculateHeadersRanges(
        XAPICallTranscript.recv.message,
        redactionItem.response.headers,
      );

      expect(result).toEqual([
        getHeaderRange(XAPICallTranscript.recv, "status"),
        getHeaderRange(XAPICallTranscript.recv, "expires"),
      ]);
    });

    test("all response headers", () => {
      const redactionItem = {
        response: {
          headers: fixtureAllResponseHeaders,
        },
      };

      const result = calculateHeadersRanges(
        XAPICallTranscript.recv.message,
        redactionItem.response.headers,
      );

      expect(result).toEqual(allResponseHeadersRedactedRanges);
    });

    test("no response headers", () => {
      const redactionItem = {
        response: {
          headers: [],
        },
      };

      const result = calculateHeadersRanges(
        XAPICallTranscript.recv.message,
        redactionItem.response.headers,
      );

      expect(result).toEqual([]);
    });

    test("response headers_except", () => {
      const redactionItem = {
        response: {
          headers_except: fixtureAllResponseHeaders.filter(
            (header) => !["date", "content-type"].includes(header),
          ),
        },
      };

      const result = calculateHeadersRangesExcept(
        XAPICallTranscript.recv,
        redactionItem.response.headers_except,
      );

      expect(result).toEqual([
        {
          start: 23,
          end: 52,
        },
        {
          start: 175,
          end: 205,
        },
      ]);
    });

    test("all response headers with headers_except", () => {
      const redactionItem = {
        response: {
          headers_except: [],
        },
      };

      const result = calculateHeadersRangesExcept(
        XAPICallTranscript.recv,
        redactionItem.response.headers_except,
      );

      expect(result).toEqual(allResponseHeadersRedactedRanges);
    });

    test("non-existent response header", () => {
      const redactionItem: RedactResponseHeaders = {
        response: {
          headers: ["non-existent"],
        },
      };
      expect(() =>
        calculateHeadersRanges(
          XAPICallTranscript.recv.message,
          redactionItem.response.headers,
        ),
      ).toThrow(HeaderNotFoundError);
    });
  });

  describe("request headers", () => {
    test("single request header", () => {
      const redactionItem = {
        request: {
          headers: ["accept-encoding"],
        },
      };

      const result = calculateHeadersRanges(
        XAPICallTranscript.sent.message,
        redactionItem.request.headers,
      );

      expect(result).toEqual([
        {
          start: 490,
          end: 498,
        },
      ]);
    });

    test("multiple request headers", () => {
      const redactionItem = {
        request: {
          headers: ["authorization", "cookie"],
        },
      };

      const result = calculateHeadersRanges(
        XAPICallTranscript.sent.message,
        redactionItem.request.headers,
      );

      expect(result).toEqual([
        {
          start: 625,
          end: 736,
        },
        {
          start: 865,
          end: 1481,
        },
      ]);
    });

    test("all request headers", () => {
      const redactionItem = {
        request: {
          headers: fixtureAllRequestHeaders,
        },
      };

      const result = calculateHeadersRanges(
        XAPICallTranscript.sent.message,
        redactionItem.request.headers,
      );

      expect(result).toEqual(allRequestHeadersRedactedRanges);
    });

    test("no request headers", () => {
      const redactionItem = {
        request: {
          headers: [],
        },
      };

      const result = calculateHeadersRanges(
        XAPICallTranscript.sent.message,
        redactionItem.request.headers,
      );

      expect(result).toEqual([]);
    });

    test("request headers_except", () => {
      const redactionItem = {
        request: {
          headers_except: fixtureAllRequestHeaders.filter(
            (header) =>
              !["x-client-transaction-id", "connection"].includes(header),
          ),
        },
      };

      const result = calculateHeadersRangesExcept(
        XAPICallTranscript.sent,
        redactionItem.request.headers_except,
      );

      expect(result).toEqual([
        {
          start: 377,
          end: 471,
        },
        {
          start: 1700,
          end: 1705,
        },
      ]);
    });

    test("all request headers with headers_except", () => {
      const redactionItem = {
        request: {
          headers_except: [],
        },
      };

      const result = calculateHeadersRangesExcept(
        XAPICallTranscript.sent,
        redactionItem.request.headers_except,
      );

      expect(result).toEqual(allRequestHeadersRedactedRanges);
    });

    test("request headers case insensitivity", () => {
      const redactionItem = {
        request: {
          headers: ["Accept-Encoding", "CoOkIe"],
        },
      };

      const result = calculateHeadersRanges(
        XAPICallTranscript.sent.message,
        redactionItem.request.headers,
      );

      expect(result).toEqual([
        {
          start: 490,
          end: 498,
        },
        {
          start: 865,
          end: 1481,
        },
      ]);
    });

    test("not existing request header", () => {
      const redactionItem = {
        request: {
          headers: ["not-existing-header"],
        },
      };

      expect(() =>
        calculateHeadersRanges(
          XAPICallTranscript.sent.message,
          redactionItem.request.headers,
        ),
      ).toThrowError(HeaderNotFoundError);
    });
  });

  describe("duplicated headers", () => {
    test("no headers", () => {
      const redactionItem = {
        response: {
          headers: [],
        },
      };

      const result = calculateHeadersRanges(
        TranscriptWithDoubleHeaders.recv.message,
        redactionItem.response.headers,
      );

      expect(result).toEqual([]);
    });

    test("not duplicated header", () => {
      const redactionItem = {
        response: {
          headers: ["content-type"],
        },
      };

      const result = calculateHeadersRanges(
        TranscriptWithDoubleHeaders.recv.message,
        redactionItem.response.headers,
      );

      expect(result).toEqual([{ start: 105, end: 136 }]);
    });

    test("duplicated header", () => {
      const redactionItem = {
        response: {
          headers: ["date"],
        },
      };

      const result = calculateHeadersRanges(
        TranscriptWithDoubleHeaders.recv.message,
        redactionItem.response.headers,
      );

      expect(result).toEqual([
        { start: 23, end: 52 },
        { start: 60, end: 89 },
      ]);
    });

    test("all headers", () => {
      const redactionItem = {
        response: {
          headers: ["date", "content-type"],
        },
      };

      const result = calculateHeadersRanges(
        TranscriptWithDoubleHeaders.recv.message,
        redactionItem.response.headers,
      );

      expect(result).toEqual([
        { start: 23, end: 52 },
        { start: 60, end: 89 },
        { start: 105, end: 136 },
      ]);
    });

    test("header duplicated 5 times", () => {
      const redactionItem = {
        request: {
          headers: ["host"],
        },
      };

      const result = calculateHeadersRanges(
        TranscriptWithDoubleHeaders.sent.message,
        redactionItem.request.headers,
      );

      expect(result).toEqual([
        { start: 31, end: 36 },
        { start: 44, end: 49 },
        { start: 57, end: 62 },
        { start: 70, end: 75 },
        { start: 83, end: 88 },
      ]);
    });

    test("headers_except duplicated", () => {
      const redactionItem = {
        response: {
          headers_except: ["date"],
        },
      };

      const result = calculateHeadersRangesExcept(
        TranscriptWithDoubleHeaders.recv,
        redactionItem.response.headers_except,
      );

      expect(result).toEqual([{ start: 105, end: 136 }]);
    });

    test("headers_except not duplicated", () => {
      const redactionItem = {
        response: {
          headers_except: ["content-type"],
        },
      };

      const result = calculateHeadersRangesExcept(
        TranscriptWithDoubleHeaders.recv,
        redactionItem.response.headers_except,
      );

      expect(result).toEqual([
        { start: 23, end: 52 },
        { start: 60, end: 89 },
      ]);
    });

    test("all headers with headers_except", () => {
      const redactionItem = {
        response: {
          headers_except: [],
        },
      };

      const result = calculateHeadersRangesExcept(
        TranscriptWithDoubleHeaders.recv,
        redactionItem.response.headers_except,
      );

      expect(result).toEqual([
        { start: 23, end: 52 },
        { start: 60, end: 89 },
        { start: 105, end: 136 },
      ]);
    });
  });
});
