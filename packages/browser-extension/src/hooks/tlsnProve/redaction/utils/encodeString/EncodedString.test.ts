import { describe, expect, test } from "vitest";
import { EncodedString } from "./EncodedString";
import { Encoding } from "./Encoding";
import { EncodingMismatchError } from "../error";

describe("EncodedStrings", () => {
  describe("indexOf", () => {
    test("with string needle", () => {
      const encodedString = new EncodedString("hello world", Encoding.UTF8);
      expect(encodedString.indexOf("world")).toEqual(6);
    });

    test("with EncodedString needle", () => {
      const haystack = new EncodedString("hello world", Encoding.UTF8);
      const needle = new EncodedString("world", Encoding.UTF8);
      expect(haystack.indexOf(needle)).toEqual(6);
    });

    test("with from parameter", () => {
      const encodedString = new EncodedString(
        "hello world hello world",
        Encoding.UTF8,
      );
      expect(encodedString.indexOf("world", 7)).toEqual(18);
    });

    test("returns -1 when not found", () => {
      const encodedString = new EncodedString("hello world", Encoding.UTF8);
      expect(encodedString.indexOf("xyz")).toEqual(-1);
    });

    test("throws on encoding mismatch", () => {
      const haystack = new EncodedString("hello world", Encoding.UTF8);
      const needle = new EncodedString("world", Encoding.UTF16);
      expect(() => haystack.indexOf(needle)).toThrow(EncodingMismatchError);
    });

    test("works with emoji characters", () => {
      const haystack = new EncodedString("Hello 👋 world! 🌎", Encoding.UTF8);
      const needle = new EncodedString("👋", Encoding.UTF8);
      expect(haystack.indexOf(needle)).toEqual(6);
    });

    test("works with accented characters", () => {
      const haystack = new EncodedString("résumé café", Encoding.UTF8);
      const needle = new EncodedString("café", Encoding.UTF8);
      expect(haystack.indexOf(needle)).toEqual(9);
    });

    test("works with Chinese characters", () => {
      const haystack = new EncodedString("你好世界", Encoding.UTF8);
      const needle = new EncodedString("世界", Encoding.UTF8);
      expect(haystack.indexOf(needle)).toEqual(6);
    });
  });
  describe("nthIndexOf", () => {
    test("returns the index of the nth occurrence of the needle", () => {
      const encodedString = new EncodedString(
        "hello world hello world",
        Encoding.UTF8,
      );
      expect(encodedString.nthIndexOf("world", 2)).toEqual(18);
    });

    test("returns -1 when the needle is not found", () => {
      const encodedString = new EncodedString("hello world", Encoding.UTF8);
      expect(encodedString.nthIndexOf("hello", 2)).toEqual(-1);
    });

    test("works with emoji characters", () => {
      const haystack = new EncodedString("Hello 👋 world! 🌎", Encoding.UTF8);
      const needle = new EncodedString("👋", Encoding.UTF8);
      expect(haystack.nthIndexOf(needle, 1)).toEqual(6);
    });
  });

  describe("split", () => {
    test("splits the string by the separator", () => {
      const encodedString = new EncodedString("hello world", Encoding.UTF8);
      expect(encodedString.split(" ").map((p) => p.toString())).toEqual([
        "hello",
        "world",
      ]);
    });

    test("works with emoji characters", () => {
      const encodedString = new EncodedString(
        "Hello 👋 world! 🌎",
        Encoding.UTF8,
      );
      expect(encodedString.split("👋").map((p) => p.toString())).toEqual([
        "Hello ",
        " world! 🌎",
      ]);
    });

    test("works with accented characters", () => {
      const encodedString = new EncodedString("résumé café", Encoding.UTF8);
      expect(encodedString.split("café").map((p) => p.toString())).toEqual([
        "résumé ",
        "",
      ]);
    });
  });

  describe("includes", () => {
    test("returns true if the string contains the needle", () => {
      const encodedString = new EncodedString("hello world", Encoding.UTF8);
      expect(encodedString.includes("world")).toEqual(true);
    });

    test("works with emoji characters", () => {
      const encodedString = new EncodedString(
        "Hello 👋 world! 🌎",
        Encoding.UTF8,
      );
      expect(encodedString.includes("👋")).toEqual(true);
    });
  });

  describe("length", () => {
    test("returns the length of the string", () => {
      const encodedString = new EncodedString("hello world", Encoding.UTF8);
      expect(encodedString.length).toEqual(11);
    });

    test("works with emoji characters", () => {
      const encodedString = new EncodedString(
        "Hello 👋 world! 🌎",
        Encoding.UTF8,
      );
      expect(encodedString.length).toEqual(22);
    });

    test("works with accented characters", () => {
      const encodedString = new EncodedString("résumé café", Encoding.UTF8);
      expect(encodedString.length).toEqual(14);
    });
  });

  describe("slice", () => {
    test("returns a slice of the string", () => {
      const encodedString = new EncodedString("hello world", Encoding.UTF8);
      expect(encodedString.slice(0, 5)).toEqual(
        new EncodedString("hello", Encoding.UTF8),
      );
    });

    test("works with emoji characters", () => {
      const encodedString = new EncodedString(
        "Hello 👋 world! 🌎",
        Encoding.UTF8,
      );
      expect(encodedString.slice(0, 5)).toEqual(
        new EncodedString("Hello", Encoding.UTF8),
      );
    });

    test("works with accented characters", () => {
      const encodedString = new EncodedString("résumé café", Encoding.UTF8);
      expect(encodedString.slice(0, 5)).toEqual(
        new EncodedString("résu", Encoding.UTF8),
      );
    });

    describe("caseInsensitiveIndexOf", () => {
      test("finds exact match with emojis", () => {
        const str = new EncodedString("Hello 👋 World 🌎", Encoding.UTF8);
        expect(str.caseInsensitiveIndexOf("Hello 👋")).toBe(0);
      });

      test("finds case insensitive match with special characters", () => {
        const str = new EncodedString("Café 🍵 RÉSUMÉ", Encoding.UTF8);
        expect(str.caseInsensitiveIndexOf("café")).toBe(0);
        expect(str.caseInsensitiveIndexOf("rÉSuMÉ")).toBe(11);
      });

      test("returns -1 when emoji not found", () => {
        const str = new EncodedString("Hello 👋 World 🌎", Encoding.UTF8);
        expect(str.caseInsensitiveIndexOf("🎉")).toBe(-1);
      });

      test("works with string containing only emojis", () => {
        const str = new EncodedString("🎮🎲🎯🎪", Encoding.UTF8);
        expect(str.caseInsensitiveIndexOf("🎲")).toBe(4);
      });

      test("works with mixed characters", () => {
        const str = new EncodedString(
          "你好 Hello こんにちは 👋",
          Encoding.UTF8,
        );
        expect(str.caseInsensitiveIndexOf("HELLO")).toBe(7);
      });
    });
  });
});
