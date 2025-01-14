import { RedactionConfig } from "src/web-proof-commons/types/message";
import { redact } from "./redact";
import { describe, expect, test } from "vitest";
import { fixtureTranscript } from "./tlsn.ranges.test.fixtures";
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

    const result = redact(mockTranscript, redactionConfig);

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

    const result = redact(mockTranscript, redactionConfig);

    expect(result.sent).toEqual([
      {
        start:
          mockTranscript.ranges.sent.headers["x-client-transaction-id"].start +
          "x-client-transaction-id".length +
          1,
        end: 471,
      }, // x-client-transaction-id
      {
        start:
          mockTranscript.ranges.sent.headers["accept-encoding"].start +
          "accept-encoding".length +
          1,
        end: 498,
      }, // accept-encoding
      {
        start:
          mockTranscript.ranges.sent.headers["sec-ch-ua"].start +
          "sec-ch-ua".length +
          1,
        end: 576,
      }, // sec-ch-ua
      {
        start:
          mockTranscript.ranges.sent.headers["content-type"].start +
          "content-type".length +
          1,
        end: 608,
      }, // content-type
      {
        start:
          mockTranscript.ranges.sent.headers["authorization"].start +
          "authorization".length +
          1,
        end: 736,
      }, // authorization
      {
        start:
          mockTranscript.ranges.sent.headers["sec-ch-ua-mobile"].start +
          "sec-ch-ua-mobile".length +
          1,
        end: 775,
      }, // sec-ch-ua-mobile
      {
        start:
          mockTranscript.ranges.sent.headers["accept"].start +
          "accept".length +
          1,
        end: 788,
      }, // accept
      {
        start:
          mockTranscript.ranges.sent.headers["x-twitter-auth-type"].start +
          "x-twitter-auth-type".length +
          1,
        end: 824,
      }, // x-twitter-auth-type
      {
        start:
          mockTranscript.ranges.sent.headers["x-twitter-client-language"]
            .start +
          "x-twitter-client-language".length +
          1,
        end: 855,
      }, // x-twitter-client-language
      {
        start:
          mockTranscript.ranges.sent.headers["cookie"].start +
          "cookie".length +
          1,
        end: 1481,
      }, // cookie
      {
        start:
          mockTranscript.ranges.sent.headers["sec-ch-ua-platform"].start +
          "sec-ch-ua-platform".length +
          1,
        end: 1510,
      }, // sec-ch-ua-platform
      {
        start:
          mockTranscript.ranges.sent.headers["x-csrf-token"].start +
          "x-csrf-token".length +
          1,
        end: 1686,
      }, // x-csrf-token
      {
        start:
          mockTranscript.ranges.sent.headers["connection"].start +
          "connection".length +
          1,
        end: 1705,
      }, // connection
      {
        start:
          mockTranscript.ranges.sent.headers["x-twitter-active-user"].start +
          "x-twitter-active-user".length +
          1,
        end: 1733,
      }, // x-twitter-active-user
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

    const result = redact(mockTranscript, redactionConfig);

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

    const result = redact(mockTranscript, redactionConfig);

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
    const result = redact(mockTranscript, []);

    expect(result).toEqual({ sent: [], recv: [] });
  });
});
