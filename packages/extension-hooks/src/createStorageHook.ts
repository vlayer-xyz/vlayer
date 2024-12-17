import {
  useState,
  useEffect,
  useCallback,
  type Dispatch,
  type SetStateAction,
} from "react";
import browser from "webextension-polyfill";
import { LOADING } from "./constants";

function createStorageHook(storage: browser.Storage.StorageArea) {
  return function useStorage<T>(
    storageKey: string,
    initialValue?: T,
  ): [T | typeof LOADING, Dispatch<SetStateAction<T | typeof LOADING>>] {
    const [storedValue, setStoredValue] = useState<T | typeof LOADING>(LOADING);
    useEffect(() => {
      storage
        .get(storageKey)
        .then((result) => {
          if (result[storageKey] !== undefined) {
            setStoredValue(result[storageKey] as T);
          } else {
            setStoredValue(initialValue as T);
          }
        })
        .catch(console.error);
    }, [storageKey, JSON.stringify(initialValue)]);

    useEffect(() => {
      const handleStorageChange = (changes: {
        [key: string]: browser.Storage.StorageChange;
      }) => {
        Object.entries(changes).forEach(([key, change]) => {
          if (key === storageKey) {
            setStoredValue((change.newValue || initialValue) as T);
          }
        });
      };
      storage.onChanged.addListener(handleStorageChange);

      return () => {
        storage.onChanged.removeListener(handleStorageChange);
      };
    }, [storageKey]);

    const setValue: Dispatch<SetStateAction<T | typeof LOADING>> = useCallback(
      (newValue) => {
        const updatedValue =
          typeof newValue === "function"
            ? (newValue as (x: T | typeof LOADING) => T | undefined)(
                storedValue,
              )
            : newValue;
        setStoredValue(newValue);
        storage.set({ [storageKey]: updatedValue }).catch(console.error);
      },
      [storageKey, storage, storedValue],
    );

    return [storedValue, setValue];
  };
}

export default createStorageHook;
