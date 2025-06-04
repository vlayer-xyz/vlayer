import { describe, it, expect } from "vitest";
import type { WebProofStepUserAction } from "../../web-proof-commons";
import { domStateAssertion } from "./domStateAssertions";

describe("domStateAssertion", () => {
  it("returns true if element is present and we expect it", () => {
    const element = document.createElement("div");
    const assertion = {
      domElement: "div",
      require: { exist: true },
    } as WebProofStepUserAction["assertion"];
    expect(domStateAssertion(element, assertion)).toBe(true);
  });

  it("returns true if element is not present and we expect it not to exist", () => {
    const element = null;
    const assertion = {
      domElement: "div",
      require: { notExist: true },
    } as WebProofStepUserAction["assertion"];
    expect(domStateAssertion(element, assertion)).toBe(true);
  });

  it("returns false if element is not present but we expect it to exist", () => {
    const element = null;
    const assertion = {
      domElement: "div",
      require: { exist: true },
    } as WebProofStepUserAction["assertion"];
    expect(domStateAssertion(element, assertion)).toBe(false);
  });

  it("returns false if element is present but we expect it not to exist", () => {
    const element = document.createElement("div");
    const assertion = {
      domElement: "div",
      require: { notExist: true },
    } as WebProofStepUserAction["assertion"];
    expect(domStateAssertion(element, assertion)).toBe(false);
  });
});
