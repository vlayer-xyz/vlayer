import { renderHook } from "@testing-library/react";
import { describe, test, expect, vi, beforeEach } from "vitest";
import { useSyncChain } from "./useSyncChain";

import { anvil, optimismSepolia } from "viem/chains";

const mocks = vi.hoisted(() => {
  return {
    useAccount: vi.fn(),
    useSwitchChain: vi.fn(),
    useChainId: vi.fn(),
    useChains: vi.fn(),
  };
});
describe("useChain with anvil", () => {
  beforeEach(() => {
    vi.mock("wagmi", () => ({
      ...mocks,
    }));

    mocks.useAccount.mockReturnValue({
      address: "0x123",
      chainId: anvil.id,
    });
    mocks.useSwitchChain.mockImplementation(() => {
      return {
        switchChain: vi.fn(),
      };
    });
    mocks.useChainId.mockReturnValue(anvil.id);
    mocks.useChains.mockReturnValue([optimismSepolia, anvil]);
  });

  test("env and wallet chain match", () => {
    const chainFromEnv = "anvil";

    const { result } = renderHook(() => useSyncChain(chainFromEnv));
    const chain = result.current.chain;

    expect(result.current.error).toBeNull();
    expect(chain).toBeDefined();
    expect(chain?.name).toBe("Anvil");
  });

  test("fail with meaningful error when switch fails", () => {
    mocks.useSwitchChain.mockImplementationOnce(() => {
      return {
        switchChain: vi
          .fn()
          .mockImplementation(
            (_chainId: number, { onError }: { onError: () => void }) => {
              onError();
            },
          ),
      };
    });
    const mismatchedChainFromEnv = "sepolia";

    const { result } = renderHook(() => useSyncChain(mismatchedChainFromEnv));
    const error = result.current.error;

    expect(result.current.chain).toBeNull();
    expect(error).toBeDefined();
    expect(error?.message).toBe(
      "Failed to switch to Sepolia make sure you have it in your wallet",
    );
  });

  test("Indicate when chain is switched", () => {
    mocks.useSwitchChain.mockImplementationOnce(() => {
      return {
        switchChain: vi
          .fn()
          .mockImplementation(
            (_chainId: number, { onSuccess }: { onSuccess: () => void }) => {
              onSuccess();
            },
          ),
      };
    });
    const chainFromEnv = "optimismSepolia";

    const { result } = renderHook(() => useSyncChain(chainFromEnv));
    const chain = result.current.chain;

    expect(result.current.error).toBeNull();
    expect(chain).toBeDefined();
    expect(result.current.switched).toBe(true);
  });

  test("fail with meaningful error when env chain is undefined", () => {
    // Simulates the case when the env chain is not set
    const { result } = renderHook(() => useSyncChain(undefined));
    const error = result.current.error;

    expect(result.current.chain).toBeNull();
    expect(error).toBeDefined();
    expect(error?.message).toBe("Env chain not defined");
  });

  test("fail with meaningful error when chain is not supported by viem", () => {
    const unsupportedChainFromEnv = "unsupported-chain";

    const { result } = renderHook(() => useSyncChain(unsupportedChainFromEnv));
    const error = result.current.error;

    expect(result.current.chain).toBeNull();
    expect(error).toBeDefined();
    expect(error?.message).toBe("Chain unsupported-chain is not supported");
  });
});
