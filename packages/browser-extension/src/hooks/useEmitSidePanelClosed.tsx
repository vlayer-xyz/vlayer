import { useEffect } from "react";
import { ExtensionMessageType } from "src/web-proof-commons";
import browser from "webextension-polyfill";

export const useEmitSidePanelClosed = () => {
  useEffect(() => {
    const handleBeforeUnload = () => {
      void browser.runtime.sendMessage(ExtensionMessageType.SidePanelClosed);
    };

    window.addEventListener("beforeunload", handleBeforeUnload);

    return () => {
      window.removeEventListener("beforeunload", handleBeforeUnload);
    };
  }, []);
};
