import { renderHook } from "@testing-library/react";
import { ExtensionInternalMessageType } from "src/web-proof-commons";
import browser from "webextension-polyfill";
import { useResetTlsnSessionOnNewWebproofRequest } from "./useResetTlsnSessionOnRequest";
import { describe, it, expect, vi, type Mock, beforeEach } from "vitest";

const mockResetTlsnProving = vi.fn();

vi.mock("./useTlsnProver", () => ({
  useTlsnProver: () => ({
    resetTlsnProving: mockResetTlsnProving,
  }),
}));

describe("useResetTlsnSessionOnNewWebproofRequest", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });
  it("should add message listener on mount", () => {
    renderHook(() => useResetTlsnSessionOnNewWebproofRequest());
    // eslint-disable-next-line @typescript-eslint/unbound-method
    expect(browser.runtime.onMessage.addListener).toHaveBeenCalledTimes(1);
  });

  it("should remove message listener on unmount", () => {
    const { unmount } = renderHook(() =>
      useResetTlsnSessionOnNewWebproofRequest(),
    );
    unmount();
    // eslint-disable-next-line @typescript-eslint/unbound-method
    expect(browser.runtime.onMessage.removeListener).toHaveBeenCalledTimes(1);
  });

  it("should call resetTlsnProving when receiving ResetTlsnProving message", () => {
    renderHook(() => useResetTlsnSessionOnNewWebproofRequest());
    const listener = (browser.runtime.onMessage.addListener as Mock).mock
      .calls[0][0] as (message: unknown) => void;
    listener({ type: ExtensionInternalMessageType.ResetTlsnProving });
    expect(mockResetTlsnProving).toHaveBeenCalledTimes(1);
  });

  it("should not call resetTlsnProving for other message types", () => {
    renderHook(() => useResetTlsnSessionOnNewWebproofRequest());
    const listener = (browser.runtime.onMessage.addListener as Mock).mock
      .calls[0][0] as (message: unknown) => void;
    listener("SomeOtherMessageType");
    expect(mockResetTlsnProving).not.toHaveBeenCalled();
  });
});
