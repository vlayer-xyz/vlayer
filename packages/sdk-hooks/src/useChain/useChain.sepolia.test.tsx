import { renderHook } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { useChain } from "./useChain";

import { anvil, optimismSepolia } from "viem/chains";

describe("useChain with optimism sepolia", () => {
  beforeEach(() => {
    vi.mock("wagmi", () => ({
      useChainId: () => optimismSepolia.id,
      useChains: () => [optimismSepolia, anvil],
    }));
  });

  it("should successfully return chain", () => {
    const chainFromEnv = "optimismSepolia";

    const { result } = renderHook(() => useChain(chainFromEnv));
    const chain = result.current.chain;

    expect(result.current.error).toBeNull();
    expect(chain).toBeDefined();
    expect(chain?.name).toBe("OP Sepolia");
  });

  it("should fail when the wrong name is used in env", () => {
    const mismatchedChainFromEnv = "OP Sepolia";

    const { result } = renderHook(() => useChain(mismatchedChainFromEnv));
    const error = result.current.error;

    expect(result.current.chain).toBeNull();
    expect(error).toBeDefined();
    expect(error).toBe("Chain OP Sepolia is not supported");
  });
});
