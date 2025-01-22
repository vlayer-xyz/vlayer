import { RedactResponseHeaders } from "src/web-proof-commons/types/message";
import { describe, expect, test } from "vitest";
import {
  getHeaderRange,
  XAPICallTranscript,
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
});
