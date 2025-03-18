import { describe, expect, test } from "vitest";
import { getSize } from "./getSize";

describe("getSize", () => {
  test("returns correct size for a non-empty string", () => {
    const value = "Hello, World!";
    const size = getSize(value);
    expect(size).toBe(new TextEncoder().encode(value).length);
  });

  test("returns correct size for an empty string", () => {
    const value = "";
    const size = getSize(value);
    expect(size).toBe(new TextEncoder().encode(value).length);
  });

  test("returns correct size for undefined", () => {
    const value = undefined;
    const size = getSize(value);
    expect(size).toBe(new TextEncoder().encode(value).length);
  });
});
