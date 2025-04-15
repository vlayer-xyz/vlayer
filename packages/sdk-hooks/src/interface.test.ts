import { describe, it, expect } from "vitest";
import * as hooks from "./index";

describe("interface", () => {
  it("should export all must have hooks", () => {
    expect(Object.keys(hooks)).toEqual(expect.arrayContaining(["useWebProof"]));
  });
});
