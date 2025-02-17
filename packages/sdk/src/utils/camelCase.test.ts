import { describe, expect, expectTypeOf, test } from "vitest";
import { toCamelCase, keysToCamelCase } from "./camelCase";

describe("toCamelCase", () => {
  test("with empty string", () => {
    expect(toCamelCase("")).toBe("");
  });

  test("correctly converts SCREAMING_CASE to camelCase", () => {
    expect(toCamelCase("SCREAMING")).toBe("screaming");
    expect(toCamelCase("SCREAMING_CASE")).toBe("screamingCase");
    expect(toCamelCase("SCREAMING___--_CASE")).toBe("screamingCase");
  });
});

describe("keysToCamelCase", () => {
  test("with empty object", () => {
    expect(keysToCamelCase({})).toEqual({});
  });

  test("converts object keys to camelCase", () => {
    expect(
      keysToCamelCase({
        SCREAMING_CASE: 1,
        HELLO_WORLD: 2,
      }),
    ).toEqual({
      screamingCase: 1,
      helloWorld: 2,
    });
  });

  test("does not do deep conversion", () => {
    expect(
      keysToCamelCase({
        SCREAMING_CASE: {
          HELLO_WORLD: 1,
        },
      }),
    ).toEqual({
      screamingCase: {
        HELLO_WORLD: 1,
      },
    });
  });

  test("infers correct type", () => {
    expectTypeOf(
      keysToCamelCase({
        SCREAMING_CASE: 1,
        HELLO_WORLD: "hello",
      }),
    ).toEqualTypeOf<{
      screamingCase: number;
      helloWorld: string;
    }>();
  });

  test("does not convert non-string keys", () => {
    expect(
      keysToCamelCase({
        1: 1,
        2: 2,
      }),
    ).toEqual({
      1: 1,
      2: 2,
    });
  });
});
