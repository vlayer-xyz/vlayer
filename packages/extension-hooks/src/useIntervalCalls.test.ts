import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { renderHook, act } from "@testing-library/react";
import { useIntervalCalls } from "./useIntervalCalls";

describe("useIntervalCalls", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.clearAllTimers();
    vi.restoreAllMocks();
  });

  async function execMicrotask() {
    await act(async () => {
      await Promise.resolve();
    });
  }

  it("calls callback automatically after timeout", async () => {
    const cb = vi.fn();

    renderHook(() => useIntervalCalls(cb, 1000));
    expect(cb).toHaveBeenCalledOnce();

    await execMicrotask();
    vi.advanceTimersByTime(1000);
    await execMicrotask();
    expect(cb).toHaveBeenCalledTimes(2);
    vi.advanceTimersByTime(999);
    await execMicrotask();
    expect(cb).toHaveBeenCalledTimes(2);
    vi.advanceTimersByTime(1);
    await execMicrotask();
    expect(cb).toHaveBeenCalledTimes(3);
  });

  it("clears timer on unmount", async () => {
    const cb = vi.fn();
    const { unmount } = renderHook(() => useIntervalCalls(cb, 1000));

    expect(cb).toHaveBeenCalledOnce();

    unmount();
    vi.advanceTimersByTime(2000);
    await execMicrotask();

    expect(cb).toHaveBeenCalledOnce();
  });
});
