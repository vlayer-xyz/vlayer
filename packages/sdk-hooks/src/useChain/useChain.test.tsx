import { renderHook } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { useChain } from "./useChain";

import { sepolia, anvil } from "viem/chains";

describe("useChain", () => {
  beforeEach(() => {
    vi.mock("wagmi", () => ({
      useChainId: () => anvil.id,
      useChains: () => [sepolia, anvil],
    }));
  });

  it("should successfully return chain", () => {
    vi.stubEnv("VITE_VLAYER_CHAIN_ID", "anvil");

    const { result } = renderHook(() => useChain());
    const chain = result.current.chain;

    expect(result.current.error).toBeUndefined();
    expect(chain).toBeDefined();
    expect(chain).toBe("anvil");
  });

  it("should fail when chains mismatched", () => {
    vi.stubEnv("VITE_VLAYER_CHAIN_ID", "sepolia");

    const { result } = renderHook(() => useChain());
    const error = result.current.error;

    expect(result.current.chain).toBeUndefined();
    expect(error).toBeDefined();
    expect(error).toBe(
      "Chains mismatched. Wallet chain: anvil is not equal to env chain: sepolia",
    );
  });

  it("should fail when env chain is undefined", () => {
    vi.unstubAllEnvs();

    const { result } = renderHook(() => useChain());
    const error = result.current.error;

    expect(result.current.chain).toBeUndefined();
    expect(error).toBeDefined();
    expect(error).toBe("Env chain undefined not found");
  });

  it("should fail when chain is not supported", () => {
    vi.stubEnv("VITE_VLAYER_CHAIN_ID", "unsupported-chain");

    const { result } = renderHook(() => useChain());
    const error = result.current.error;

    expect(result.current.chain).toBeUndefined();
    expect(error).toBeDefined();
    expect(error).toBe("Chain unsupported-chain is not suported");
  });
});
