/* eslint-disable @typescript-eslint/require-await */
import { renderHook, act, waitFor } from "@testing-library/react";
import browser from "webextension-polyfill";
import { describe, afterEach, it, expect } from "vitest";
import createStorageHook from "./createStorageHook";
import { LOADING } from "./constants";
//this works same for sync and session storage
const localStorage = browser.storage.local;
const useLocalStorage = createStorageHook(localStorage);

describe(useLocalStorage, () => {
  afterEach(async () => {
    await browser.storage.local.clear();
  });

  it("should retrieve an LOADING before it loads value from store", async () => {
    await act(() => localStorage.set({ foo: "bar" }));
    const { result } = renderHook(() => useLocalStorage("foo", "initialValue"));
    const [state] = result.current;
    expect(state).toEqual(LOADING);
  });

  it("should retrieve actual value when there is something in store", async () => {
    await act(() => localStorage.set({ foo: "bar" }));
    const { result } = renderHook(() => useLocalStorage("foo"));
    await waitFor(() => {
      expect(result.current[0]).not.toBe(LOADING);
    });
    const [state] = result.current;
    expect(state).toEqual("bar");
  });

  it("should retrieve initial value when there is nothing in store", async () => {
    const { result } = renderHook(() => useLocalStorage("foo", "initialValue"));
    await waitFor(() => {
      expect(result.current[0]).not.toBe(LOADING);
    });
    const [state] = result.current;
    expect(state).toEqual("initialValue");
  });

  it("should prioritise actual value over initial state", async () => {
    await act(() => localStorage.set({ foo: "bar" }));
    const { result } = renderHook(() => useLocalStorage("foo", "baz"));
    await waitFor(() => {
      expect(result.current[0]).not.toBe(LOADING);
    });
    const [state] = result.current;
    expect(state).toEqual("bar");
  });

  it("correctly updates localStorage", async () => {
    const { result } = renderHook(() => useLocalStorage("foo", "bar"));
    const [, setFoo] = result.current;
    await act(async () => setFoo("baz"));
    expect((await localStorage.get("foo"))["foo"]).toEqual("baz");
  });

  it("should return undefined if no initialValue provided and localStorage empty", async () => {
    const { result } = renderHook(() => useLocalStorage("some_key"));
    await waitFor(() => {
      expect(result.current[0]).not.toBe(LOADING);
    });
    expect(result.current[0]).toBeUndefined();
  });

  it("should properly return new value ", async () => {
    const { result } = renderHook(() => useLocalStorage("foo", "bar"));
    await waitFor(() => {
      expect(result.current[0]).not.toBe(LOADING);
    });
    const [, setFoo] = result.current;
    await act(async () => setFoo("baz"));
    const [foo] = result.current;
    expect(foo).toEqual("baz");
  });

  it("should keep in sync two hooks based on the same key", async () => {
    await act(() => localStorage.set({ foo: "bar" }));
    const { result: r1 } = renderHook(() => useLocalStorage("foo"));
    const { result: r2 } = renderHook(() => useLocalStorage("foo"));
    await waitFor(() => {
      expect(r1.current[0]).not.toBe(LOADING);
    });
    await waitFor(() => {
      expect(r2.current[0]).not.toBe(LOADING);
    });
    const [, setFoo] = r1.current;
    await act(async () => setFoo("baz"));
    const [val1] = r1.current;
    const [val2] = r2.current;
    expect(val1).toEqual(val2);
    expect(val2).toEqual("baz");
  });

  it("should work with function updater", async () => {
    const { result } = renderHook(() => useLocalStorage("foo", "bar"));
    await waitFor(() => {
      expect(result.current[0]).not.toBe(LOADING);
    });
    const [, setFoo] = result.current;
    await act(async () => setFoo((state) => `${state?.toString()}_bar`));
    const [value] = result.current;
    expect(value).toEqual("bar_bar");
  });
});
