import { renderHook, act } from "@testing-library/react";
import { useMessage } from "./useMessage";
import { describe, test, expect } from "vitest";
import browser from "webextension-polyfill";
describe("useMessagePayload", () => {
  test("updates state when message with matching topic is received", async () => {
    const message = { payload: { data: "test" }, topic: "test-topic" };
    const { result } = renderHook(() => useMessage(message.topic));
    await act(async () => {
      await browser.runtime.sendMessage(message);
    });
    expect(result.current).toEqual(message.payload);
  });

  test("does not update state when message with non-matching topic is received", async () => {
    const message = { payload: { data: "test" }, topic: "test-topic" };
    const { result } = renderHook(() => useMessage("other-topic"));
    await act(async () => {
      await browser.runtime.sendMessage(message);
    });
    expect(result.current).toBeNull();
  });
});
