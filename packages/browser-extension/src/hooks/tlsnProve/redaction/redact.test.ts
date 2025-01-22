import { RedactionConfig } from "src/web-proof-commons/types/message";
import { calcRedactionRanges, calcRevealRanges } from "./redact";
import { describe, expect, test } from "vitest";
import {
  extractHeaders,
  XAPICallTranscript,
} from "./tlsn.ranges.test.fixtures";
import { InvalidRangeError } from "./utils";
import { OutOfBoundsError } from "./utils";

describe("redact tests", () => {
  describe("calcRevealRanges", () => {
    test("invalid range", () => {
      const wholeTranscriptRange = {
        start: 0,
        end: 100,
      };
      const toRedact = [
        {
          start: 50,
          end: 40,
        },
      ];
      expect(() => calcRevealRanges(wholeTranscriptRange, toRedact)).toThrow(
        InvalidRangeError,
      );
    });
    
    test("disjoint intervals", () => {
      const wholeTranscriptRange = {
        start: 0,
        end: 100,
      };
      const toRedact = [
        {
          start: 110,
          end: 120,
        },
      ];

      expect(() => calcRevealRanges(wholeTranscriptRange, toRedact)).toThrow(
        OutOfBoundsError,
      );
    });

    test("overlapping ranges", () => {
      const wholeTranscriptRange = {
        start: 0,
        end: 100,
      };
      const toRedact = [
        {
          start: 10,
          end: 20,
        },
        {
          start: 10,
          end: 40,
        },
      ];

      expect(() => calcRevealRanges(wholeTranscriptRange, toRedact)).toThrow(
        InvalidRangeError,
      );
    });
    test("disjoint ranges", () => {
      const wholeTranscriptRange = {
        start: 0,
        end: 100,
      };
      const toRedact = [
        {
          start: 10,
          end: 20,
        },
        {
          start: 30,
          end: 40,
        },
      ];

      const result = calcRevealRanges(wholeTranscriptRange, toRedact);

      expect(result).toEqual([
        {
          start: 0,
          end: 10,
        },
        {
          start: 20,
          end: 30,
        },
        {
          start: 40,
          end: 100,
        },
      ]);
    });

    test("empty redact ranges", () => {
      const wholeTranscriptRange = {
        start: 0,
        end: 100,
      };
      const result = calcRevealRanges(wholeTranscriptRange, []);

      expect(result).toEqual([{ start: 0, end: 100 }]);
    });

    test("single redact range", () => {
      const wholeTranscriptRange = {
        start: 0,
        end: 100,
      };
      const result = calcRevealRanges(wholeTranscriptRange, [
        { start: 10, end: 20 },
      ]);

      expect(result).toEqual([
        { start: 0, end: 10 },
        { start: 20, end: 100 },
      ]);
    });

    test("multiple redact ranges starting at the beginning of transcript", () => {
      const wholeTranscriptRange = {
        start: 0,
        end: 100,
      };
      const result = calcRevealRanges(wholeTranscriptRange, [
        { start: 0, end: 20 },
        { start: 30, end: 100 },
      ]);

      expect(result).toEqual([{ start: 20, end: 30 }]);
    });
  });

  describe("redact", () => {
    const mockTranscript = XAPICallTranscript;

    test("redacts request headers", () => {
      const redactionConfig: RedactionConfig = [
        {
          request: {
            headers: ["authorization"],
          },
        },
      ];

      const result = calcRedactionRanges(mockTranscript, redactionConfig);

      const start =
        mockTranscript.sent.message.content.indexOf("authorization") +
        "authorization".length +
        2;
      const end = mockTranscript.sent.message.content.indexOf("\r\n", start);
      expect(result.sent).toEqual([
        {
          start,
          end,
        },
      ]);
    });

    test("redacts request headers except specified ones", () => {
      const redactionConfig: RedactionConfig = [
        {
          request: {
            headers_except: extractHeaders(
              mockTranscript.sent.message.content.toString(),
            ).filter(
              (header) => header !== "host" && header !== "authorization",
            ),
          },
        },
      ];

      const values = {
        host: "api.x.com",
        authorization:
          "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA",
      };
      const result = calcRedactionRanges(mockTranscript, redactionConfig);

      expect(result.sent).toEqual([
        {
          start: mockTranscript.sent.message.content.indexOf(
            values.authorization,
          ),
          end:
            mockTranscript.sent.message.content.indexOf(values.authorization) +
            values.authorization.length,
        },
        {
          start: mockTranscript.sent.message.content.nthIndexOf(values.host, 2),
          end:
            mockTranscript.sent.message.content.nthIndexOf(values.host, 2) +
            values.host.length,
        },
      ]);

      expect(result.recv).toEqual([]);
    });

    test("redacts response headers", () => {
      const redactionConfig: RedactionConfig = [
        {
          response: {
            headers: ["x-transaction", "content-type"],
          },
        },
      ];

      const result = calcRedactionRanges(mockTranscript, redactionConfig);

      expect(result.sent).toEqual([]);
      expect(result.recv).toEqual([
        {
          start:
            mockTranscript.recv.message.content.indexOf("f7370b3d41b0ce46"),
          end:
            mockTranscript.recv.message.content.indexOf("f7370b3d41b0ce46") +
            "f7370b3d41b0ce46".length,
        },
        {
          start: mockTranscript.recv.message.content.indexOf(
            "application/json;charset=utf-8",
          ),
          end:
            mockTranscript.recv.message.content.indexOf(
              "application/json;charset=utf-8",
            ) + "application/json;charset=utf-8".length,
        },
      ]);
    });

    test("handles multiple redaction items", () => {
      const redactionConfig: RedactionConfig = [
        {
          response: {
            headers: ["pragma", "content-type"],
          },
        },
        {
          response: {
            json_body: ["screen_name", "mention_filter"],
          },
        },
        {
          request: {
            headers: ["accept-encoding"],
          },
        },
      ];

      const result = calcRedactionRanges(mockTranscript, redactionConfig);

      expect(result.recv).toEqual([
        {
          start: mockTranscript.recv.message.content.indexOf("no-cache"),
          end:
            mockTranscript.recv.message.content.indexOf("no-cache") +
            "no-cache".length,
        },
        {
          start: mockTranscript.recv.message.content.indexOf(
            "application/json;charset=utf-8",
          ),
          end:
            mockTranscript.recv.message.content.indexOf(
              "application/json;charset=utf-8",
            ) + "application/json;charset=utf-8".length,
        },
        {
          start: mockTranscript.recv.message.content.indexOf("g_p_vlayer"),
          end:
            mockTranscript.recv.message.content.indexOf("g_p_vlayer") +
            "g_p_vlayer".length,
        },
        {
          start: mockTranscript.recv.message.content.indexOf("unfiltered"),
          end:
            mockTranscript.recv.message.content.indexOf("unfiltered") +
            "unfiltered".length,
        },
      ]);
      expect(result.sent).toEqual([
        {
          start: mockTranscript.sent.message.content.indexOf("identity"),
          end:
            mockTranscript.sent.message.content.indexOf("identity") +
            "identity".length,
        },
      ]);
    });

    test("returns empty commit for empty redaction config", () => {
      const result = calcRedactionRanges(mockTranscript, []);

      expect(result).toEqual({ sent: [], recv: [] });
    });
  });
});
