import { useState, useEffect, useCallback, type Dispatch, type SetStateAction} from "react";
import browser from "webextension-polyfill";
import { LOADING } from "./constants";

type HookValue<T> = (T | typeof LOADING | undefined); 
type SetHookValue<T> = ((value: HookValue<T> ) => T)| T;  


function createStorageHook(storage: browser.Storage.StorageArea) {
  return function useStorage<T>(
    storageKey: string,
    initialValue?: T,
  ): [HookValue<T>, Dispatch<SetStateAction<HookValue<T>>>] {
    
    const [storedValue, setStoredValue] = useState<HookValue<T>>(LOADING);

    useEffect(() => {
      storage
        .get(storageKey)
        .then((result) => {
          if (result[storageKey] !== undefined) {
            setStoredValue(result[storageKey] as T);
          } else {
            setStoredValue(initialValue);
          }
        })
        .catch(console.error);
    }, [storageKey, initialValue]);

    useEffect(() => {
      const handleStorageChange = (changes: {
        [key: string]: browser.Storage.StorageChange;
      }) => {
        Object.entries(changes).forEach(([key, change]) => {
          if (key === storageKey) {
            setStoredValue(change.newValue as T);
          }
        });
      };

      storage.onChanged.addListener(handleStorageChange);

      return () => {
        storage.onChanged.removeListener(handleStorageChange);
      };
    }, [storageKey]);

    const setValue : Dispatch<SetStateAction<HookValue<T>>>  = useCallback(
      async (newValue) => {
        const updatedValue = typeof newValue === "function" ? (newValue as Function)(storedValue) : newValue;
        setStoredValue(newValue); 
        await storage.set({ [storageKey]: updatedValue });
      },
      [storageKey, storage, storedValue],
    );

    return [storedValue, setValue];
  };
}

export default createStorageHook;
