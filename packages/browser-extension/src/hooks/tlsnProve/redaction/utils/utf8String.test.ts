import { describe, expect, test } from "vitest";
import { utf8IndexOf, Utf8String } from "./utf8String";

describe("utf8IndexOf", () => {
  test("returns 0 when needle is empty", () => {
    const haystack = new Uint8Array([1, 2, 3]);
    const needle = new Uint8Array([]);
    expect(utf8IndexOf(haystack, needle)).toBe(0);
  });

  test("returns -1 when needle is longer than haystack", () => {
    const haystack = new Uint8Array([1, 2]);
    const needle = new Uint8Array([1, 2, 3]);
    expect(utf8IndexOf(haystack, needle)).toBe(-1);
  });

  test("finds needle at start of haystack", () => {
    const haystack = new Uint8Array([1, 2, 3, 4]);
    const needle = new Uint8Array([1, 2]);
    expect(utf8IndexOf(haystack, needle)).toBe(0);
  });

  test("finds needle in middle of haystack", () => {
    const haystack = new Uint8Array([1, 2, 3, 4]);
    const needle = new Uint8Array([2, 3]);
    expect(utf8IndexOf(haystack, needle)).toBe(1);
  });

  test("finds needle at end of haystack", () => {
    const haystack = new Uint8Array([1, 2, 3, 4]);
    const needle = new Uint8Array([3, 4]);
    expect(utf8IndexOf(haystack, needle)).toBe(2);
  });

  test("returns -1 when needle is not found", () => {
    const haystack = new Uint8Array([1, 2, 3, 4]);
    const needle = new Uint8Array([5, 6]);
    expect(utf8IndexOf(haystack, needle)).toBe(-1);
  });

  test("respects from parameter", () => {
    const haystack = new Uint8Array([1, 2, 1, 2]);
    const needle = new Uint8Array([1, 2]);
    expect(utf8IndexOf(haystack, needle, 2)).toBe(2);
  });
});

describe("Utf8String split", () => {
  test("splits string correctly", () => {
    const utf8String = new Utf8String("hello:world");
    expect(utf8String.split(":")).toEqual([
      new Utf8String("hello"),
      new Utf8String("world"),
    ]);
  });
});

describe("Utf8String includes", () => {
  test("returns true when needle is found", () => {
    const utf8String = new Utf8String("hello:world");
    expect(utf8String.includes("hello")).toBe(true);
  });
});

describe("Utf8String equals", () => {
  test("returns true when strings are equal", () => {
    const utf8String = new Utf8String("hello");
    expect(utf8String.equals(new Utf8String("hello"))).toBe(true);
  });
});

describe("Utf8String slice", () => {
  test("slices string correctly", () => {
    const utf8String = new Utf8String("hello:world");
    expect(utf8String.slice(0, 5)).toEqual(new Utf8String("hello"));
  });
});

describe("Utf8String toUtf16String", () => {
  test("returns the string", () => {
    const utf8String = new Utf8String("hello");
    expect(utf8String.toUtf16String()).toEqual("hello");
  });
});

describe("Utf8String nthIndexOf", () => {
  test("returns the index of the nth occurrence of the needle", () => {
    const utf8String = new Utf8String("hello:world:hello:world");
    expect(utf8String.nthIndexOf("hello", 2)).toEqual(12);
  });

  test("returns -1 when the needle is not found", () => {
    const utf8String = new Utf8String("hello:people:hello");
    expect(utf8String.nthIndexOf("world", 2)).toEqual(-1);
  });

  test("returns -1 when the nth is greater than the number of occurrences", () => {
    const utf8String = new Utf8String("hello:world:hello");
    expect(utf8String.nthIndexOf("hello", 3)).toEqual(-1);
  });
});
