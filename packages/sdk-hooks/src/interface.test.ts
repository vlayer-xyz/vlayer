import { describe, it, expect } from "vitest";
import { hooks } from "./index";

describe("interface", () => {
  it("should export all must have hooks", () => {
    expect(Object.keys(hooks)).toEqual(
      expect.arrayContaining([
        "useWebProof", // counterpart of webProofProvider.getWebProof https://book.vlayer.xyz/javascript/web-proofs.html
        "useProveWeb", // counterpart of vlayerClient.proveWeb
        "useProve", // counterpart of vlayerClient.prove
      ]),
    );
  });

  it("should export all nice to have hooks", () => {
    expect(Object.keys(hooks)).toEqual(
      expect.arrayContaining([
        // representing verification on chain
        "useVerify",
        // representing full flow of webproof from extention to on chain verification
        "useProveVerifyWeb",
        // representing interction with vlayer contracts ( prove + verify) having webproof in hand
        "useProveVerify",
      ]),
    );
  });
});
