import { useState, useEffect, useCallback } from "react";
import browser from "webextension-polyfill";
function createStorageHook(storage: browser.Storage.StorageArea) {
  // for now this implementation is enough
  // to add later :
  // - serialize and deserialize the value
  // - syn WebStorage support

  return function useStorage<T>(
    storageKey: string,
    initialValue: T,
  ): [T, (newValue: T) => Promise<void>] {
    const [storedValue, setStoredValue] = useState<T>(initialValue);

    useEffect(() => {
      storage.set({ [storageKey]: initialValue });

      // obser storage changes

      const handleStorageChange = (changes: {
        [key: string]: browser.Storage.StorageChange;
      }) => {
        Object.entries(changes).forEach(([key, change]) => {
          if (key === storageKey) {
            setStoredValue(change.newValue as T);
          }
        });
      };

      browser.storage.onChanged.addListener(handleStorageChange);

      return () => {
        browser.storage.onChanged.removeListener(handleStorageChange);
      };
    }, []);

    // sync storage value with state

    const setValue = useCallback(
      async (newValue: T) => {
        setStoredValue(newValue);
        await storage.set({ [storageKey]: newValue });
      },
      [storageKey, storage],
    );

    return [storedValue, setValue];
  };
}

export default createStorageHook;
