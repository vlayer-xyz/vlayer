import { renderHook, act } from "@testing-library/react";
import { describe, it, expect, vi, beforeEach } from "vitest";
import {
  useWaitForProvingResult,
  WaitForProvingResultStatus,
} from "./useWaitForProvingResult";
import type { BrandedHash } from "@vlayer/sdk";
import type { Abi } from "viem";

const mockVlayerClient = vi.hoisted(() => ({
  waitForProvingResult: vi.fn(),
}));

describe("useWaitForProvingResult", () => {
  const mockHash = "0x123" as unknown as BrandedHash<Abi, string>;
  const mockResult = { success: true };
  const mockError = new Error("Test error");

  vi.mock("../context", () => ({
    useProofContext: () => ({
      vlayerClient: mockVlayerClient,
    }),
  }));

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("should initialize properly", () => {
    const { result } = renderHook(() => useWaitForProvingResult(mockHash));

    expect(result.current.status).toBe(WaitForProvingResultStatus.Idle);
    expect(result.current.isIdle).toBe(true);
    expect(result.current.error).toBeNull();
  });

  it("should handle successful proving result", async () => {
    mockVlayerClient.waitForProvingResult.mockResolvedValueOnce(mockResult);

    const { result } = renderHook(() => useWaitForProvingResult(mockHash));

    await act(async () => {
      await result.current.waitForProvingResult();
    });
    expect(result.current.data).toEqual(mockResult);
    expect(result.current.status).toBe(WaitForProvingResultStatus.Ready);
    expect(result.current.isReady).toBe(true);
    expect(result.current.error).toBeNull();
    expect(mockVlayerClient.waitForProvingResult).toHaveBeenCalledWith({
      hash: mockHash,
    });
  });

  it("should handle proving error", async () => {
    mockVlayerClient.waitForProvingResult.mockRejectedValueOnce(mockError);

    const { result } = renderHook(() => useWaitForProvingResult(mockHash));

    await act(async () => {
      await result.current.waitForProvingResult();
    });

    expect(result.current.status).toBe(WaitForProvingResultStatus.Error);
    expect(result.current.isError).toBe(true);
    expect(result.current.error).toBe(mockError);
  });

  it("should set pending status while waiting for result", () => {
    mockVlayerClient.waitForProvingResult.mockImplementation(
      () =>
        new Promise((resolve) => setTimeout(() => resolve(mockResult), 100)),
    );

    const { result } = renderHook(() => useWaitForProvingResult(mockHash));

    act(() => {
      void result.current.waitForProvingResult();
    });

    expect(result.current.status).toBe(WaitForProvingResultStatus.Pending);
    expect(result.current.isPending).toBe(true);
  });
});
