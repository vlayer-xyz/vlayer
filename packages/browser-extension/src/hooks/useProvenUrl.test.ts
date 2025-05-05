import { renderHook } from "@testing-library/react";
import { LOADING } from "@vlayer/extension-hooks";
import { describe, test, expect, vi, beforeEach } from "vitest";
import { useProvenUrl } from "./useProvenUrl";

const mocks = vi.hoisted(() => {
  return {
    useProvingSessionConfig: vi.fn(),
    useBrowsingHistory: vi.fn(),
  };
});

vi.mock("./useProvingSessionConfig", () => ({
  useProvingSessionConfig: mocks.useProvingSessionConfig,
}));

vi.mock("./useBrowsingHistory", () => ({
  useBrowsingHistory: mocks.useBrowsingHistory,
}));

describe("useProvenUrl", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });
  test("returns null when config is loading", () => {
    mocks.useProvingSessionConfig.mockImplementation(() => [LOADING]);
    mocks.useBrowsingHistory.mockImplementation(() => [[]]);
    const { result } = renderHook(() => useProvenUrl());
    expect(result.current).toBeNull();
  });

  test("returns null when no notarize step exists", () => {
    mocks.useProvingSessionConfig.mockImplementation(() => [
      {
        steps: [{ step: "startPage", url: "http://example.com" }],
      },
    ]);
    mocks.useBrowsingHistory.mockImplementation(() => [[]]);

    const { result } = renderHook(() => useProvenUrl());
    expect(result.current).toBeNull();
  });
  test("returns matching history item when URL matches notarize step", () => {
    mocks.useProvingSessionConfig.mockImplementation(() => [
      {
        steps: [{ step: "notarize", url: "http://example.com/api" }],
      },
    ]);
    mocks.useBrowsingHistory.mockImplementation(() => [
      [{ url: "http://example.com/api", ready: true }],
    ]);

    const { result } = renderHook(() => useProvenUrl());
    expect(result.current).toEqual({
      url: "http://example.com/api",
      ready: true,
    });
  });

  test("returns matching history item when URL pattern matches with additional path segments", () => {
    mocks.useProvingSessionConfig.mockImplementation(() => [
      {
        steps: [{ step: "notarize", url: "http://example.com/api/*" }],
      },
    ]);
    mocks.useBrowsingHistory.mockImplementation(() => [
      [{ url: "http://example.com/api/additional/path", ready: true }],
    ]);

    const { result } = renderHook(() => useProvenUrl());
    expect(result.current).toEqual({
      url: "http://example.com/api/additional/path",
      ready: true,
    });
  });

  test("picks step based on both url pattern and method", () => {
    mocks.useBrowsingHistory.mockImplementation(() => [
      [
        {
          url: "http://example.com/api/additional/path",
          ready: true,
          method: "GET",
        },
        {
          url: "http://example.com/api/additional/path",
          ready: true,
          method: "POST",
        },
      ],
    ]);

    function testForMethod(method: "POST" | "GET") {
      mocks.useProvingSessionConfig.mockImplementation(() => [
        {
          steps: [
            { step: "notarize", url: "http://example.com/api/*", method },
          ],
        },
      ]);
      const { result } = renderHook(() => useProvenUrl());
      expect(result.current).toEqual({
        method,
        url: "http://example.com/api/additional/path",
        ready: true,
      });
    }

    testForMethod("POST");
    testForMethod("GET");
  });

  test("returns null when URL doesn't match notarize step", () => {
    mocks.useBrowsingHistory.mockImplementation(() => [
      [{ url: "http://different.com" }],
    ]);

    const { result } = renderHook(() => useProvenUrl());
    expect(result.current).toBeNull();
  });
});
