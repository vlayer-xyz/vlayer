import { act, renderHook } from "@testing-library/react";
import { it, expect, describe, vi, beforeEach } from "vitest";
import { useWebProof } from "./useWebProof";
import { ProofProvider } from "../context";
import { MockExtensionWebProofProvider } from "./extension.mock";
import { WebProofRequestStatus } from "../types";

const mocks = vi.hoisted(() => ({
  createExtensionWebProofProvider: vi.fn(),
}));

vi.mock("@vlayer/sdk/web_proof", () => ({
  createExtensionWebProofProvider: mocks.createExtensionWebProofProvider,
}));

const stubWebProofRequest = {
  proverCallCommitment: {
    address: "0x" as `0x${string}`,
    proverAbi: [
      {
        type: "function",
        name: "mint",
        inputs: [],
        outputs: [],
        stateMutability: "nonpayable",
      },
    ] as const,
    chainId: 1,
    functionName: "mint",
    commitmentArgs: [] as [],
  },
  logoUrl: "http://twitterswap.com/logo.png",
  steps: [],
};

describe("useWebproof", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  const VlayerMockContext = ({ children }: { children: React.ReactNode }) => (
    <ProofProvider>{children}</ProofProvider>
  );

  const renderWebproofHook = () =>
    renderHook(() => useWebProof(stubWebProofRequest), {
      wrapper: VlayerMockContext,
    });

  const setupMockProvider = (options = {}) => {
    mocks.createExtensionWebProofProvider.mockImplementation(
      () => new MockExtensionWebProofProvider(options),
    );
    vi.runAllTimers();
  };

  it("should throw error if called outside of ProofProvider", () => {
    expect(() => renderHook(() => useWebProof(stubWebProofRequest))).toThrow(
      "useProofContext must be used within a ProofProvider",
    );
  });

  it("should initialize with the correct state", () => {
    setupMockProvider();
    const { result } = renderWebproofHook();

    expect(result.current).toEqual({
      webProof: null,
      status: WebProofRequestStatus.idle,
      isIdle: true,
      isError: false,
      isSuccess: false,
      isPending: false,
      error: null,
      requestWebProof: expect.any(Function) as () => void,
    });
  });

  it("should request the web proof", () => {
    const { result } = renderWebproofHook();
    act(() => result.current.requestWebProof());

    expect(result.current).toEqual({
      webProof: null,
      status: WebProofRequestStatus.pending,
      isIdle: false,
      isError: false,
      isSuccess: false,
      isPending: true,
      error: null,
      requestWebProof: expect.any(Function) as () => void,
    });
  });

  it("should set the web proof when the request is successful", () => {
    setupMockProvider({
      shouldSucceed: true,
      mockProof: { mock: "proof" },
    });
    const { result, rerender } = renderWebproofHook();

    act(() => result.current.requestWebProof());
    vi.advanceTimersByTime(1000);
    rerender();

    expect(result.current).toMatchObject({
      webProof: {
        webProofJson: JSON.stringify({ presentationJson: { mock: "proof" } }),
      },
      status: WebProofRequestStatus.success,
      isIdle: false,
      isPending: false,
      isError: false,
      isSuccess: true,
      error: null,
      requestWebProof: expect.any(Function) as () => void,
    });
  });

  it("should set the error when the request fails", () => {
    setupMockProvider({
      shouldSucceed: false,
      mockError: "Mock error occurred",
    });
    const { result, rerender } = renderWebproofHook();

    act(() => result.current.requestWebProof());
    vi.advanceTimersByTime(1000);
    rerender();

    expect(result.current).toEqual({
      webProof: null,
      status: WebProofRequestStatus.error,
      isIdle: false,
      isPending: false,
      isError: true,
      isSuccess: false,
      error: new Error("Mock error occurred"),
      requestWebProof: expect.any(Function) as () => void,
    });
  });
});
