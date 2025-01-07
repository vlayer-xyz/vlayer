import { InvalidZkProvingStatus, useZkProvingState } from "./useZkProvingState";
import { describe, expect, it, vi } from "vitest";
import { ZkProvingStatus } from "../web-proof-commons";
import { renderHook } from "@testing-library/react";
import { LOADING } from "@vlayer/extension-hooks";

const mocks = vi.hoisted(() => {
  return {
    useLocalStorage: vi.fn(),
    useSessionStorage: vi.fn(),
    LOADING: "loading",
  };
});

vi.mock("@vlayer/extension-hooks", () => ({
  LOADING: "loading",
  useLocalStorage: mocks.useLocalStorage,
  useSessionStorage: mocks.useSessionStorage,
}));

describe("useZkProvingState", () => {
  it("should return ZkProvingStatus.NotStarted when loading value from storage", () => {
    mocks.useLocalStorage.mockImplementation(() => [LOADING]);
    mocks.useSessionStorage.mockImplementation(() => [LOADING]);
    const { result } = renderHook(() => useZkProvingState());
    expect(result.current.value).toBe(ZkProvingStatus.NotStarted);
  });

  it("should indicate that value is wrong", () => {
    mocks.useLocalStorage.mockImplementation(() => ["somethingStrange"]);
    mocks.useSessionStorage.mockImplementation(() => ["somethingStrange"]);
    const { result } = renderHook(() => useZkProvingState());
    expect(result.current.isError).toBe(true);
    expect(result.current.error).toBeInstanceOf(InvalidZkProvingStatus);
  });

  Object.values(ZkProvingStatus).forEach((status) => {
    it(`should return ${status} when storage is set to ${status}`, () => {
      mocks.useLocalStorage.mockImplementation(() => [status]);
      mocks.useSessionStorage.mockImplementation(() => [status]);
      const { result } = renderHook(() => useZkProvingState());
      expect(result.current.value).toBe(status);
    });
  });
});
