import { useEffect } from "react";
import {
  isExtensionInternalMessage,
  ExtensionInternalMessageType,
} from "src/web-proof-commons";
import browser from "webextension-polyfill";
import { useTlsnProver } from "./useTlsnProver";

export const useResetTlsnSessionOnNewWebproofRequest = () => {
  const { resetTlsnProving } = useTlsnProver();
  useEffect(() => {
    const listener = (message: unknown) => {
      if (
        isExtensionInternalMessage(message) &&
        message.type === ExtensionInternalMessageType.ResetTlsnProving
      ) {
        resetTlsnProving();
      }
    };
    browser.runtime.onMessage.addListener(listener);
    return () => {
      browser.runtime.onMessage.removeListener(listener);
    };
  }, [resetTlsnProving]);
};
