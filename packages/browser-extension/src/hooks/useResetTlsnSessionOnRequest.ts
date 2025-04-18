import { useEffect } from "react";
import {
  MessageToExtension,
  MessageToExtensionType,
} from "src/web-proof-commons";
import browser from "webextension-polyfill";
import { useTlsnProver } from "./useTlsnProver";

export const useResetTlsnSessionOnNewWebproofRequest = () => {
  const { resetTlsnProving } = useTlsnProver();
  useEffect(() => {
    const listener = (message: MessageToExtension) => {
      if (message.type === MessageToExtensionType.RequestWebProof) {
        resetTlsnProving();
      }
    };
    browser.runtime.onMessage.addListener(listener);
    return () => {
      browser.runtime.onMessage.removeListener(listener);
    };
  }, [resetTlsnProving]);
};
