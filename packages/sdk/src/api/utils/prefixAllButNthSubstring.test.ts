import { describe, expect, test } from "vitest";
import { prefixAllButNthSubstring } from "./prefixAllButNthSubstring";

describe("prefixAllButNthSubstring", () => {
  test("adds 'X-' prefix to all matches except n-th (indexed from 0)", () => {
    const str = "abc 123 abc 456 abc 789";
    expect(prefixAllButNthSubstring(str, /abc/gi, 3, 0)).toBe(
      "abc 123 X-abc 456 X-abc 789",
    );
    expect(prefixAllButNthSubstring(str, /abc/gi, 3, 1)).toBe(
      "X-abc 123 abc 456 X-abc 789",
    );
    expect(prefixAllButNthSubstring(str, /abc/gi, 3, 2)).toBe(
      "X-abc 123 X-abc 456 abc 789",
    );
  });

  test("does not add prefix to substrings past total substring count", () => {
    const str = "abc 123 abc 456 abc 789 abc abc";
    expect(prefixAllButNthSubstring(str, /abc/gi, 3, 1)).toBe(
      "X-abc 123 abc 456 X-abc 789 abc abc",
    );
  });
});
