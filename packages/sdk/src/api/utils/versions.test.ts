import { describe, test, expect } from "vitest";
import { checkVersionCompatibility } from "./versions";

describe("versions compatibility", () => {
  test("throws if major version mismatches", () => {
    expect(() => {
      checkVersionCompatibility("1.2.3", "2.1.3");
    }).toThrowError(
      "SDK version 2.1.3 is incompatible with prover version 1.2.3",
    );
  });

  test("throws if major version is 0 and minor version mismatches", () => {
    expect(() => {
      checkVersionCompatibility("0.2.3", "0.1.3");
    }).toThrowError(
      "SDK version 0.1.3 is incompatible with prover version 0.2.3",
    );
  });

  test("does not throw if major and minor versions match", () => {
    expect(() => {
      checkVersionCompatibility("1.2.3", "1.2.13");
    }).not.toThrow();
  });

  test("does not throw if major version is >0 and minor mismatches", () => {
    expect(() => {
      checkVersionCompatibility("1.2.3", "1.5.8");
    }).not.toThrow();
  });

  test("does not throw if major version is 0 and minor matchs", () => {
    expect(() => {
      checkVersionCompatibility("0.2.3", "0.2.7");
    }).not.toThrow();
  });
});
