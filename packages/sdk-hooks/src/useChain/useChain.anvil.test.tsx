import { renderHook } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { useChain } from "./useChain";

import { sepolia, anvil } from "viem/chains";

describe("useChain with anvil", () => {
  beforeEach(() => {
    vi.mock("wagmi", () => ({
      useChainId: () => anvil.id,
      useChains: () => [sepolia, anvil],
    }));
  });

  it("should successfully return chain", () => {
    const chainFromEnv = "anvil";

    const { result } = renderHook(() => useChain(chainFromEnv));
    const chain = result.current.chain;

    expect(result.current.error).toBeNull();
    expect(chain).toBeDefined();
    expect(chain?.name).toBe("Anvil");
  });

  it("should fail when chains mismatched", () => {
    const mismatchedChainFromEnv = "sepolia";

    const { result } = renderHook(() => useChain(mismatchedChainFromEnv));
    const error = result.current.error;

    expect(result.current.chain).toBeNull();
    expect(error).toBeDefined();
    expect(error).toBe(
      "Chains mismatched. Wallet chain: Anvil is not equal to env chain: sepolia",
    );
  });

  it("should fail when env chain is undefined", () => {
    // Simulates the case when the env chain is not set
    const { result } = renderHook(() => useChain(undefined));
    const error = result.current.error;

    expect(result.current.chain).toBeNull();
    expect(error).toBeDefined();
    expect(error).toBe("Env chain not defined");
  });

  it("should fail when chain is not supported", () => {
    const unsupportedChainFromEnv = "unsupported-chain";

    const { result } = renderHook(() => useChain(unsupportedChainFromEnv));
    const error = result.current.error;

    expect(result.current.chain).toBeNull();
    expect(error).toBeDefined();
    expect(error).toBe("Chain unsupported-chain is not supported");
  });
});
