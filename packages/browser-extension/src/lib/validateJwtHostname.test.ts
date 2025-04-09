import { describe, test, expect } from "vitest";
import { validateJwtHostname } from "./validateJwtHostname";

describe("basic JWT validation", () => {
  test("should validate JWT hostname", () => {
    expect(() =>
      validateJwtHostname(
        { host: "x.com", port: 443, sub: "test", exp: 1234 },
        "y.com",
      ),
    ).toThrowError("Invalid JWT hostname");
    expect(
      validateJwtHostname(
        { host: "x.com", port: 443, sub: "test", exp: 1234 },
        "x.com",
      ),
    ).toEqual("x.com");
  });
});
