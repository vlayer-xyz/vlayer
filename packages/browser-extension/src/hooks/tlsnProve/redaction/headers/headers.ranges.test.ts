import { RedactResponseHeaders } from "src/web-proof-commons/types/message";
import { describe, expect, test } from "vitest";
import {
  getHeaderRange,
  XAPICallTranscript,
} from "../test.fixtures";
import { calculateHeadersRanges } from "./headers.ranges";
import { HeaderNotFoundError } from "../utils/error";

describe("response headers", () => {
  test("single header", () => {
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
  test("multiple headers", () => {
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
  test("non-existent header", () => {
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
