import { useEffect } from "react";
import { provingSessionStorageConfig } from "src/state/config";
import { ExtensionMessageType } from "src/web-proof-commons";
import browser from "webextension-polyfill";

// Listen to clean storage request where window is available

export const useCleanStorageOnClose = () => {
  useEffect(() => {
    browser.runtime.onMessage.addListener((message) => {
      if (message === ExtensionMessageType.CleanProvingSessionStorageOnClose) {
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
