import { useState } from "react";
import { useEffect } from "react";
import browser from "webextension-polyfill";

export const useMessage = <T = unknown>(topic: T) => {
  const [state, setState] = useState<object | null>(null);
  useEffect(() => {
    const listener = (runtimeMessage: { payload: object; topic: T }) => {
      if (topic === runtimeMessage.topic) {
        setState(runtimeMessage.payload);
      }
    };
    browser.runtime.onMessage.addListener(listener);
    return () => {
      browser.runtime.onMessage.removeListener(listener);
    };
  }, [topic]);
  return state;
};
