import { afterEach, describe, expect, test, vi } from "vitest";
import { renderHook } from "@testing-library/react";
import { useAddress } from "./useAddress.ts";

const envPrivateKey =
  "0x1111111111111111111111111111111111111111111111111111111111111111";
const addressFromPrivateKey = "0x19E7E376E7C213B7E7e7e46cc70A5dD086DAff2A";
const invalidPrivateKey = "0x1234";

const addressFromAppKit = "0x9999999999999999999999999999999999999999";
const mockAppKitAccountResponse = {
  address: addressFromAppKit,
};
const mockNoWalletResponse = { address: undefined };

const mocks = vi.hoisted(() => {
  return {
    useAppKitAccount: vi.fn(),
  };
});

vi.mock("@reown/appkit/react", () => {
  return {
    useAppKitAccount: mocks.useAppKitAccount,
  };
});

describe("useAddress", () => {
  afterEach(() => {
    vi.unstubAllEnvs();
    mocks.useAppKitAccount.mockRestore();
  });

  test("should return the address based on private key", () => {
    vi.stubEnv("VITE_CLIENT_AUTH_MODE", "envPrivateKey");
    vi.stubEnv("VITE_PRIVATE_KEY", envPrivateKey);

    const { result } = renderHook(() => useAddress());

    expect(result.current.address).toEqual(addressFromPrivateKey);
    expect(result.current.error).toBeNull();
  });

  test("should throw an error if no private key in env", () => {
    vi.stubEnv("VITE_CLIENT_AUTH_MODE", "envPrivateKey");
    vi.stubEnv("VITE_PRIVATE_KEY", undefined);

    const { result } = renderHook(() => useAddress());

    expect(result.current.address).toEqual("");
    expect(result.current.error).toEqual("No private key found");
  });

  test("should throw an error if invalid private key in env", () => {
    vi.stubEnv("VITE_CLIENT_AUTH_MODE", "envPrivateKey");
    vi.stubEnv("VITE_PRIVATE_KEY", invalidPrivateKey);

    const { result } = renderHook(() => useAddress());

    expect(result.current.address).toEqual("");
    expect(result.current.error).toEqual(
      "invalid private key, expected hex or 32 bytes, got string",
    );
  });

  test("should return the address from wallet", () => {
    vi.stubEnv("VITE_CLIENT_AUTH_MODE", "wallet");
    mocks.useAppKitAccount.mockReturnValue(mockAppKitAccountResponse);

    const { result } = renderHook(() => useAddress());

    expect(result.current.address).toEqual(addressFromAppKit);
    expect(result.current.error).toBeNull();
  });

  test("should throw an error if no address found in wallet", () => {
    vi.stubEnv("VITE_CLIENT_AUTH_MODE", "wallet");
    mocks.useAppKitAccount.mockReturnValue(mockNoWalletResponse);

    const { result } = renderHook(() => useAddress());

    expect(result.current.address).toEqual("");
    expect(result.current.error).toEqual("No address found in wallet");
  });

  test("should throw an error if invalid VITE_CLIENT_AUTH_MODE", () => {
    vi.stubEnv("VITE_CLIENT_AUTH_MODE", "invalid");

    const { result } = renderHook(() => useAddress());

    expect(result.current.address).toEqual("");
    expect(result.current.error).toEqual("Invalid VITE_CLIENT_AUTH_MODE");
  });
});
