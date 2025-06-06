import { describe, expect, test } from "vitest";
import { filterOverrides } from "./filterOverrides";

describe("filterOverrides", () => {
  test("correctly filter env overrides that were not defined", () => {
    expect(filterOverrides({ a: "d", b: "c" })).toEqual({ a: "d", b: "c" });
    expect(filterOverrides({ a: undefined, b: "c" })).toEqual({ b: "c" });
  });
});
