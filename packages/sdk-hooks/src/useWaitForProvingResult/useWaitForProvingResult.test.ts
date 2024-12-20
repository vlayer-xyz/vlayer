import { renderHook, waitFor } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import {
  useWaitForProvingResult,
  WaitForProvingResultStatus,
} from "./useWaitForProvingResult";
import type { BrandedHash } from "@vlayer/sdk";
import type { Abi } from "viem";

const mockResult = { success: true };
const mockError = new Error("Test error");
const mockHash = "0x123";

const mockVlayerClient = vi.hoisted(() => ({
  waitForProvingResult: vi.fn().mockResolvedValue({ success: true }),
}));
vi.mock("../context", () => ({
  useProofContext: () => ({
    vlayerClient: mockVlayerClient,
  }),
}));

describe("useWaitForProvingResult", () => {
  it("should initialize properly", () => {
    const { result } = renderHook(() => useWaitForProvingResult(null));

    expect(result.current).toMatchObject({
      status: WaitForProvingResultStatus.Idle,
      isIdle: true,
      error: null,
    });
  });

  it("should handle successful proving result", async () => {
    const { result } = renderHook(() =>
      useWaitForProvingResult({ hash: "0x1234" } as BrandedHash<Abi, string>),
    );

    await waitFor(() => {
      expect(result.current).toMatchObject({
        data: mockResult,
        status: WaitForProvingResultStatus.Ready,
        isReady: true,
        error: null,
      });
    });
  });
  it("should handle proving error", async () => {
    mockVlayerClient.waitForProvingResult.mockRejectedValueOnce(mockError);

    const { result } = renderHook(() =>
      useWaitForProvingResult({ hash: mockHash } as BrandedHash<Abi, string>),
    );

    await waitFor(() => {
      expect(result.current).toMatchObject({
        status: WaitForProvingResultStatus.Error,
        isError: true,
        error: mockError,
      });
    });
  });

  it("should set pending status while waiting for result", async () => {
    mockVlayerClient.waitForProvingResult.mockImplementation(
      () =>
        new Promise((resolve) => setTimeout(() => resolve(mockResult), 100)),
    );

    const { result } = renderHook(() =>
      useWaitForProvingResult({ hash: mockHash } as BrandedHash<Abi, string>),
    );
    await waitFor(() => {
      expect(result.current).toMatchObject({
        status: WaitForProvingResultStatus.Pending,
        isPending: true,
      });
    });
  });
});
