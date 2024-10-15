import browser from "webextension-polyfill";
import createStorageHook from "./createStorageHook";

export const useSessionStorage = createStorageHook(browser.storage.session);
