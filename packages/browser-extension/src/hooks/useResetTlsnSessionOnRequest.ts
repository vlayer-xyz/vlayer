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
    browser.runtime.onMessage.addListener((message: MessageToExtension) => {
      if (message.type === MessageToExtensionType.RequestWebProof) {
        resetTlsnProving();
      }
    });
  }, [resetTlsnProving]);
};
