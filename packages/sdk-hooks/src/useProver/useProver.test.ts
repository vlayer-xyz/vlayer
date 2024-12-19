import { renderHook, act } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import { useCallProver, ProverStatus } from "./useProver";
import type { Abi } from "viem";
import type { ProveArgs } from "@vlayer/sdk";

const mockVlayerClient = vi.hoisted(() => ({
  prove: vi.fn(),
}));

const mockChainId = 1;
describe("useCallProver", () => {
  beforeEach(() => {
    vi.mock("../context", () => ({
      useProofContext: () => ({
        vlayerClient: mockVlayerClient,
      }),
    }));

    vi.mock("wagmi", () => ({
      useChainId: () => mockChainId,
    }));

    mockVlayerClient.prove.mockReset();
  });

  it("should initialize properly", () => {
    const { result } = renderHook(() => useCallProver());

    expect(result.current.status).toBe(ProverStatus.Idle);
    expect(result.current.error).toBeNull();
    expect(result.current.data.hash).toBe("");
    expect(result.current.isIdle).toBe(true);
    expect(result.current.isPending).toBe(false);
    expect(result.current.isReady).toBe(false);
    expect(result.current.isError).toBe(false);
  });

  it("should handle successful prover call", async () => {
    const mockHash = "0x123";
    mockVlayerClient.prove.mockResolvedValueOnce({ hash: mockHash });

    const { result } = renderHook(() => useCallProver());

    await act(async () => {
      await result.current.callProver({
        address: "0x456",
        proverAbi: [] as Abi,
        functionName: "test",
        args: [],
      });
    });

    expect(result.current.status).toBe(ProverStatus.Ready);
    expect(result.current.data.hash).toBe(mockHash);
    expect(result.current.isReady).toBe(true);
    expect(result.current.error).toBeNull();
  });

  it("should handle prover call errors", async () => {
    const mockError = new Error("Proving failed");
    mockVlayerClient.prove.mockRejectedValueOnce(mockError);

    const { result } = renderHook(() => useCallProver());

    await act(async () => {
      await result.current.callProver({
        address: "0x456",
        proverAbi: {} as Abi,
        functionName: "test",
        args: [],
      });
    });

    expect(result.current.status).toBe(ProverStatus.Error);
    expect(result.current.error).toBe(mockError);
    expect(result.current.isError).toBe(true);
  });

  it("should pass chainId to vlayerClient.prove", async () => {
    const { result } = renderHook(() => useCallProver());

    const proverCallArgs: ProveArgs<Abi, string> = {
      address: "0x456",
      proverAbi: [] as Abi,
      functionName: "test",
      args: [],
    };

    await act(async () => {
      await result.current.callProver(proverCallArgs);
    });

    expect(mockVlayerClient.prove).toHaveBeenCalledWith({
      ...proverCallArgs,
      chainId: mockChainId,
    });
  });
});
