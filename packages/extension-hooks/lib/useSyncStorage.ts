import browser from "webextension-polyfill";
import createStorageHook from "./createStorageHook";

export const useSyncStorage = createStorageHook(browser.storage.sync);
