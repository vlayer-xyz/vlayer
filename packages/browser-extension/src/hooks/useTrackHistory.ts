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

        try {
          const rawBytes = details.requestBody?.raw?.[0]
            .bytes as AllowSharedBufferSource;
          if (!rawBytes) {
            console.warn("No request body bytes available");
            return;
          }

          // Try to detect encoding from content-type header if available
          // Default to UTF-8 if we can't determine the encoding
          let bodyText: string;
          try {
            const decoder = new TextDecoder("utf-8", { fatal: true });
            bodyText = decoder.decode(rawBytes);
          } catch (decodeError) {
            console.warn(
              "Failed to decode as UTF-8, falling back to ISO-8859-1",
              decodeError,
            );
            // Fallback to ISO-8859-1 (Latin1) which can decode any byte sequence
            const fallbackDecoder = new TextDecoder("iso-8859-1");
            bodyText = fallbackDecoder.decode(rawBytes);
          }

          historyContextManager
            .updateHistory({
              url: details.url,
              body: bodyText,
              method: details.method as HTTPMethod,
            })
            .catch((error) => {
              console.error("Failed to update history:", error);
            });
        } catch (error) {
          console.error("Error processing request body:", error);
        }
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
