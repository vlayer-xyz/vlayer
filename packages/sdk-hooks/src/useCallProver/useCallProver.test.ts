import { renderHook, act } from "@testing-library/react";
import { describe, test, expect, vi, beforeEach } from "vitest";
import { useCallProver, ProverStatus } from "./useCallProver";
import type { Abi } from "viem";
import type { ProveArgs } from "@vlayer/sdk";

const mockVlayerClient = vi.hoisted(() => ({
  prove: vi.fn(),
}));

const mockChainId = 1;
const anotherChainId = 2;

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

  test("initial state", () => {
    const { result } = renderHook(() =>
      useCallProver({
        address: "0x456",
        proverAbi: [] as Abi,
        functionName: "test",
      }),
    );

    expect(result.current).toMatchObject({
      status: ProverStatus.Idle,
      error: null,
      data: null,
      isIdle: true,
      isPending: false,
      isReady: false,
      isError: false,
    });
  });

  test("success", async () => {
    const mockHash = "0x123";
    mockVlayerClient.prove.mockResolvedValueOnce({ hash: mockHash });

    const { result } = renderHook(() =>
      useCallProver({
        address: "0x456",
        proverAbi: [] as Abi,
        functionName: "test",
      }),
    );

    await act(async () => {
      await result.current.callProver([]);
    });

    expect(result.current).toMatchObject({
      status: ProverStatus.Ready,
      data: { hash: mockHash },
      isReady: true,
      error: null,
      isError: false,
    });
  });

  test("error on prove failure", async () => {
    const mockError = new Error("Proving failed");
    mockVlayerClient.prove.mockRejectedValueOnce(mockError);

    const { result } = renderHook(() =>
      useCallProver({
        address: "0x456",
        proverAbi: {} as Abi,
        functionName: "test",
      }),
    );

    await act(async () => {
      await result.current.callProver([]);
    });

    expect(result.current).toMatchObject({
      status: ProverStatus.Error,
      error: mockError,
      isError: true,
    });
  });

  test("success with another chainId", async () => {
    const { result } = renderHook(() =>
      useCallProver({
        address: "0x456",
        proverAbi: [] as Abi,
        functionName: "test",
      }),
    );

    vi.mock("wagmi", () => ({
      useChainId: () => anotherChainId,
    }));

    const proverCallArgs: ProveArgs<Abi, string> = {
      address: "0x456",
      proverAbi: [] as Abi,
      functionName: "test",
      args: [],
    };

    await act(async () => {
      await result.current.callProver([]);
    });

    expect(mockVlayerClient.prove).toHaveBeenCalledWith({
      ...proverCallArgs,
      chainId: anotherChainId,
    });
  });
});
