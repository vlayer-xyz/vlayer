import { RedactionConfig } from "src/web-proof-commons/types/message";
import { calcRedactionRanges, calcRevealRanges } from "./redact";
import { describe, expect, test } from "vitest";
import { fixtureTranscript } from "./tlsn.ranges.test.fixtures";
import { InvalidRangeError } from "./tlsn.ranges.error";
import { OutOfBoundsError } from "./tlsn.ranges.error";
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
  const mockTranscript = fixtureTranscript;

  test("redacts request headers", () => {
    const redactionConfig: RedactionConfig = [
      {
        request: {
          headers: ["authorization"],
        },
      },
    ];

    const result = calcRedactionRanges(mockTranscript, redactionConfig);

    expect(result.sent).toEqual([
      {
        start:
          mockTranscript.ranges.sent.headers["authorization"].start +
          "authorization".length +
          1,
        end: mockTranscript.ranges.sent.headers["authorization"].end,
      },
    ]);
    expect(result.recv).toEqual([]);
  });

  test("redacts request headers except specified ones", () => {
    const redactionConfig: RedactionConfig = [
      {
        request: {
          headers_except: ["host", "user-agent"],
        },
      },
    ];

    const result = calcRedactionRanges(mockTranscript, redactionConfig);

    expect(result.sent).toEqual(
      Object.entries(mockTranscript.ranges.sent.headers)
        .filter(([header]) => !["host", "user-agent"].includes(header))
        .map(([header, range]) => ({
          start: range.start + header.length + 1,
          end: range.end,
        })),
    );

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
          mockTranscript.ranges.recv.headers["x-transaction"].start +
          "x-transaction".length +
          1,
        end: mockTranscript.ranges.recv.headers["x-transaction"].end,
      },
      {
        start:
          mockTranscript.ranges.recv.headers["content-type"].start +
          "content-type".length +
          1,
        end: mockTranscript.ranges.recv.headers["content-type"].end,
      },
    ]);
  });

  test("handles multiple redaction items", () => {
    const redactionConfig: RedactionConfig = [
      {
        request: {
          headers: ["authorization"],
        },
      },
      {
        response: {
          headers: ["content-type"],
        },
      },
      {
        response: {
          json_body: ["screen_name", "mention_filter"],
        },
      },
    ];

    const result = calcRedactionRanges(mockTranscript, redactionConfig);

    expect(result.sent).toEqual([
      {
        start:
          fixtureTranscript.ranges.sent.headers.authorization.start +
          "authorization".length +
          1,
        end: fixtureTranscript.ranges.sent.headers.authorization.end,
      },
    ]);
    expect(result.recv).toEqual([
      {
        start:
          fixtureTranscript.ranges.recv.headers["content-type"].start +
          "content-type".length +
          1,
        end: fixtureTranscript.ranges.recv.headers["content-type"].end,
      },
      {
        start: fixtureTranscript.recv.indexOf("g_p_vlayer"),
        end: fixtureTranscript.recv.indexOf("g_p_vlayer") + "g_p_vlayer".length,
      },
      {
        start: fixtureTranscript.recv.indexOf("unfiltered"),
        end: fixtureTranscript.recv.indexOf("unfiltered") + "unfiltered".length,
      },
    ]);
  });

  test("returns empty commit for empty redaction config", () => {
    const result = calcRedactionRanges(mockTranscript, []);

    expect(result).toEqual({ sent: [], recv: [] });
  });
});
