import { ExtensionMessageType } from "src/web-proof-commons";
import browser from "webextension-polyfill";
import { useEffect } from "react";

// Listen to close side panel request where window is available

export const useCloseSidePanelOnRequest = () => {
  useEffect(() => {
    browser.runtime.onMessage.addListener((message) => {
      if (message === ExtensionMessageType.CloseSidePanel) {
        window.close();
      }
    });
  }, []);
};
