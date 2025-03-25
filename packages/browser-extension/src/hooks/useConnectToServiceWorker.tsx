import { useEffect } from "react";
import { SIDE_PANEL_CONNECTION_NAME } from "src/constants/messaging";
import browser from "webextension-polyfill";
export const useConnectToServiceWorker = () => {
  useEffect(() => {
    browser.runtime.connect({ name: SIDE_PANEL_CONNECTION_NAME });
  }, []);
};
