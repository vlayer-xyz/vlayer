import {
  ExtensionInternalMessageType,
  isExtensionInternalMessage,
} from "src/web-proof-commons";
import browser from "webextension-polyfill";
import { useEffect } from "react";

// Listen to close side panel request where window is available

export const useCloseSidePanelOnRequest = () => {
  useEffect(() => {
    browser.runtime.onMessage.addListener((message: unknown) => {
      if (
        isExtensionInternalMessage(message) &&
        message.type === ExtensionInternalMessageType.CloseSidePanel
      ) {
        window.close();
      }
    });
  }, []);
};
