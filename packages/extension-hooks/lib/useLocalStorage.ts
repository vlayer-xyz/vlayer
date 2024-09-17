import browser from "webextension-polyfill";
import createStorageHook from "./createStorageHook";

export const useLocalStorage = createStorageHook(browser.storage.local);
