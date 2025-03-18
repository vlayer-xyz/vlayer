// Tests
import { describe, test, expect } from "vitest";
import { getBodySize } from "./body";
import { getSize } from "../utils/getSize";

describe("getBodySize", () => {
  describe("with different body types", () => {
    test("returns 0 for undefined", () => {
      const body = undefined;
      const size = getBodySize(body);
      expect(size).toBe(0);
    });

    test("returns the correct size for an empty object", () => {
      const body = JSON.stringify({});
      const size = getBodySize(body);
      expect(size).toBe(2);
    });

    test("returns the correct size for a non-empty object", () => {
      const body = JSON.stringify({ key: "value" });
      const size = getBodySize(body);
      expect(size).toBe(getSize(body));
    });

    test("returns the correct size for a nested object", () => {
      const body = JSON.stringify({ key: { nestedKey: "nestedValue" } });
      const size = getBodySize(body);
      expect(size).toBe(getSize(body));
    });

    test("returns the correct size for a deeply nested object", () => {
      const body = JSON.stringify({
        level1: { level2: { level3: { level4: "deepValue" } } },
      });
      const size = getBodySize(body);
      expect(size).toBe(getSize(body));
    });

    test("returns the correct size for an array", () => {
      const body = JSON.stringify([1, 2, 3]);
      const size = getBodySize(body);
      expect(size).toBe(getSize(body));
    });

    test("returns the correct size for a string", () => {
      const body = "test string";
      const size = getBodySize(body);
      expect(size).toBe(getSize(body));
    });
  });
});
