import { sepolia } from "viem/chains";
import { getChainSpecs } from "./getChainSpecs";
import { describe, expect, it, vi } from "vitest";

vi.mock("viem", () => {
  return {
    custom: vi.fn(() => "mocked result"),
  };
});

describe("getChainSpecs", () => {
  it("should return the correct chain object for a valid chain name", () => {
    const chain = getChainSpecs("sepolia");
    expect(chain).toEqual(sepolia);
  });

  it("should throw an error for a chain not supported by viem", () => {
    expect(() => getChainSpecs("unsupported-chain")).toThrow(
      "Chain unsupported-chain is not supported by viem",
    );
  });
});
