import { describe, expect, test } from "vitest";
import { prefixAllButNthSubstring } from "./prefixAllButNthSubstring";

describe("removeAllButNthSubstring", () => {
  test("adds 'x-' prefix to all matches except n-th (indexed from 0)", () => {
    const str = "abc 123 abc 456 abc 789";
    expect(prefixAllButNthSubstring(str, "abc", 0)).toBe(
      "abc 123 x-abc 456 x-abc 789",
    );
    expect(prefixAllButNthSubstring(str, "abc", 1)).toBe(
      "x-abc 123 abc 456 x-abc 789",
    );
    expect(prefixAllButNthSubstring(str, "abc", 2)).toBe(
      "x-abc 123 x-abc 456 abc 789",
    );
  });

  test("is case-insensitive", () => {
    const str = "abc 123 ABC 456 aBc 789";
    expect(prefixAllButNthSubstring(str, "abc", 0)).toBe(
      "abc 123 x-ABC 456 x-aBc 789",
    );
    expect(prefixAllButNthSubstring(str, "abc", 1)).toBe(
      "x-abc 123 ABC 456 x-aBc 789",
    );
    expect(prefixAllButNthSubstring(str, "abc", 2)).toBe(
      "x-abc 123 x-ABC 456 aBc 789",
    );
  });
});
