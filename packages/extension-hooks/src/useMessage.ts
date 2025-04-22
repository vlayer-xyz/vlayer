import { useState } from "react";
import { useEffect } from "react";
import browser from "webextension-polyfill";
import { z } from "zod";

const messageSchema = z.object({
  payload: z.unknown(),
  topic: z.unknown(),
});

export const useMessage = <T = unknown>(topic: T) => {
  const [state, setState] = useState<object | null>(null);
  useEffect(() => {
    const listener = (message: unknown) => {
      const result = messageSchema.safeParse(message);
      if (!result.success) {
        return;
      }
      const runtimeMessage = result.data;
      if (topic === runtimeMessage.topic) {
        setState(runtimeMessage.payload as object);
      }
    };
    browser.runtime.onMessage.addListener(listener);
    return () => {
      browser.runtime.onMessage.removeListener(listener);
    };
  }, [topic]);
  return state;
};
