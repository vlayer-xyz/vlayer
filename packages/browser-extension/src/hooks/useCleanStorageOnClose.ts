import { useEffect } from "react";
import { provingSessionStorageConfig } from "src/state/config";
import {
  ExtensionInternalMessageType,
  isExtensionInternalMessage,
} from "src/web-proof-commons";
import browser from "webextension-polyfill";

// Listen to clean storage request where window is available

export const useCleanStorageOnClose = () => {
  useEffect(() => {
    browser.runtime.onMessage.addListener((message: unknown) => {
      if (
        isExtensionInternalMessage(message) &&
        message.type ===
          ExtensionInternalMessageType.CleanProvingSessionStorageOnClose
      ) {
        window.addEventListener("beforeunload", () => {
          resetProvingSessionStorage();
        });
      }
    });
  }, []);
};

const resetProvingSessionStorage = () => {
  const keys = Object.values(provingSessionStorageConfig.storageKeys);
  for (const key of keys) {
    void browser.storage.session.remove(key);
  }
};
