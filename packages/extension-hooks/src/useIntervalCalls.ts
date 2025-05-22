import { useEffect, useRef } from "react";

export function useIntervalCalls(
  callback: () => void | Promise<void>,
  delay: number,
): void {
  const savedCallback = useRef(callback);
  const timeoutIdRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const isMountedRef = useRef<boolean>(true);

  useEffect(() => {
    savedCallback.current = callback;
  }, [callback]);

  useEffect(() => {
    isMountedRef.current = true;

    const tick = async () => {
      if (!isMountedRef.current) {
        return;
      }

      try {
        await savedCallback.current();
      } catch (error) {
        console.error("Error in useIntervalAsync callback:", error);
      }
      if (isMountedRef.current) {
        timeoutIdRef.current = setTimeout(() => void tick(), delay);
      }
    };

    void tick();

    return () => {
      isMountedRef.current = false;
      if (timeoutIdRef.current !== null) {
        clearTimeout(timeoutIdRef.current);
        timeoutIdRef.current = null;
      }
    };
  }, [delay]);
}
