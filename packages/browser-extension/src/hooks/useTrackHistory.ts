import { useCallback, useEffect } from "react";
import browser from "webextension-polyfill";
import { historyContextManager } from "../state/history";

// Record cookies of all interesting url

// Mark url as ready to use on request completion

function urlToMatchPattern(url: string): string {
  const parsedUrl = new URL(url);
  const protocol = parsedUrl.protocol;
  const domain = parsedUrl.hostname;
  const path = parsedUrl.pathname + "*";

  return `${protocol}//${domain}${path}`;
}

export const useTrackHeaders = () => {
  return useCallback((urls: string[]) => {
    browser.webRequest.onBeforeSendHeaders.addListener(
      (details) => {
        console.log("onBeforeSendHeaders", details);
        historyContextManager
          .updateHistory({
            url: details.url,
            headers: details.requestHeaders,
            tabId: details.tabId,
          })
          .catch(console.error);
      },
      {
        urls: [...urls],
      },
      ["requestHeaders"],
    );
  }, []);
};

export const useTrackCookies = () => {
  return useCallback((urls: string[]) => {
    browser.webRequest.onResponseStarted.addListener(
      (details) => {
        browser.cookies
          .getAll({ url: details.url })
          .then((cookies) => {
            historyContextManager
              .updateHistory({
                url: details.url,
                cookies,
                tabId: details.tabId,
              })
              .catch(console.error);
          })
          .catch(console.error);
      },
      { urls },
    );
  }, []);
};

export const useTrackCompleteness = () => {
  return useCallback((urls: string[]) => {
    browser.webRequest.onCompleted.addListener(
      (details) => {
        historyContextManager
          .updateHistory({
            url: details.url,
            ready: true,
            tabId: details.tabId,
          })
          .catch(console.error);
      },
      {
        urls,
      },
    );
  }, []);
};

export const useTrackTabUpdate = () => {
  return useCallback(() => {
    // eslint-disable-next-line @typescript-eslint/no-misused-promises
    browser.tabs.onUpdated.addListener(async (tabId, changeInfo, tab) => {
      if (changeInfo.status === "complete" && tab.url) {
        const history =
          (await historyContextManager.store.get("history")) || [];
        const existingItem = history.find((item) => item.tabId === tabId);
        if (existingItem) {
          historyContextManager
            .updateHistory({
              url: tab.url,
              tabId: tabId,
              ready: true,
            })
            .catch(console.error);
        }
      }
    });
  }, []);
};

export const useTrackHistory = () => {
  const trackHeaders = useTrackHeaders();
  const trackCookies = useTrackCookies();
  const trackCompleteness = useTrackCompleteness();
  const trackTabUpdate = useTrackTabUpdate();
  useEffect(() => {
    // Record headers of all interesting url
    historyContextManager
      .getUrls()
      .then((urlBases: string[]) => {
        const urls = urlBases.map(urlToMatchPattern);
        trackHeaders(urls);
        trackCookies(urls);
        trackCompleteness(urls);
        trackTabUpdate();
      })
      .catch(console.error);
  }, []);
};
