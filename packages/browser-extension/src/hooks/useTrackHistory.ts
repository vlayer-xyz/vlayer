import { useCallback, useEffect } from "react";
import browser from "webextension-polyfill";
import { historyContextManager } from "../state/history";
import { HTTPMethod } from "lib/HttpMethods";

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
        historyContextManager
          .updateHistory({
            url: details.url,
            headers: details.requestHeaders,
            method: details.method as HTTPMethod,
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
                method: details.method as HTTPMethod,
                cookies,
              })
              .catch(console.error);
          })
          .catch(console.error);
      },
      { urls },
    );
  }, []);
};
export const useTrackBody = () => {
  return useCallback((urls: string[]) => {
    browser.webRequest.onBeforeRequest.addListener(
      (details) => {
        if (
          ![HTTPMethod.POST, HTTPMethod.PUT, HTTPMethod.PATCH].includes(
            details.method as HTTPMethod,
          )
        ) {
          return;
        }
        const decoder = new TextDecoder("utf-8");
        const bodyText = decoder.decode(
          details.requestBody?.raw?.[0].bytes as AllowSharedBufferSource,
        );

        historyContextManager
          .updateHistory({
            url: details.url,
            body: bodyText,
            method: details.method as HTTPMethod,
          })
          .catch(console.error);
      },
      { urls },
      ["requestBody"],
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
            method: details.method as HTTPMethod,
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
    browser.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
      if (changeInfo.status === "complete" && tab.url) {
        historyContextManager
          .updateHistory({
            url: tab.url,
            tabId: tabId,
            method: HTTPMethod.GET,
          })
          .catch(console.error);
      }
    });
  }, []);
};

export const useTrackHistory = () => {
  const trackHeaders = useTrackHeaders();
  const trackCookies = useTrackCookies();
  const trackCompleteness = useTrackCompleteness();
  const trackTabUpdate = useTrackTabUpdate();
  const trackBody = useTrackBody();
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
        trackBody(urls);
      })
      .catch(console.error);
  }, []);
};
