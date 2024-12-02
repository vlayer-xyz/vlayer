import { describe, expect, test } from "vitest";
import { prefixAllButNthSubstring } from "./prefixAllButNthSubstring";

describe("prefixAllButNthSubstring", () => {
  test("adds 'X-' prefix to all matches except n-th (indexed from 0)", () => {
    const str = "abc 123 abc 456 abc 789";
    expect(prefixAllButNthSubstring(str, "abc", 0)).toBe(
      "abc 123 X-abc 456 X-abc 789",
    );
    expect(prefixAllButNthSubstring(str, "abc", 1)).toBe(
      "X-abc 123 abc 456 X-abc 789",
    );
    expect(prefixAllButNthSubstring(str, "abc", 2)).toBe(
      "X-abc 123 X-abc 456 abc 789",
    );
  });

  test("is case-insensitive", () => {
    const str = "abc 123 ABC 456 aBc 789";
    expect(prefixAllButNthSubstring(str, "abc", 0)).toBe(
      "abc 123 X-ABC 456 X-aBc 789",
    );
    expect(prefixAllButNthSubstring(str, "abc", 1)).toBe(
      "X-abc 123 ABC 456 X-aBc 789",
    );
    expect(prefixAllButNthSubstring(str, "abc", 2)).toBe(
      "X-abc 123 X-ABC 456 aBc 789",
    );
  });
});
