import { prove as tlsnProve } from "tlsn-js";
import browser from "webextension-polyfill";
import { useProofContext } from "./useProofContext";
import { useCallback, useEffect, useState } from "react";

// this is mechanism of extracting the headers and cookies from the request
// that works only for twitter
// we need to make it generic

const formatTlsnHeaders = (
  headers: browser.WebRequest.HttpHeadersItemType[],
  cookies: browser.Cookies.Cookie[],
  doCheck: boolean = false,
) => {
  const xCsrftoken =
    headers.find((header) => header.name === "x-csrf-token")?.value || "";
  const authToken =
    cookies.find((cookie) => cookie.name === "auth_token")?.value || "";
  const ct0 = cookies.find((cookie) => cookie.name === "ct0")?.value || "";
  const authorization =
    headers.find((header) => header.name === "authorization")?.value || "";

  if (doCheck) {
    if (!xCsrftoken) {
      throw new Error("x-csrf-token header is missing");
    }
    if (!authToken) {
      throw new Error("auth_token cookie is missing");
    }
    if (!ct0) {
      throw new Error("ct0 cookie is missing");
    }
    if (!authorization) {
      throw new Error("authorization header is missing");
    }
  }

  return {
    headers: {
      "x-twitter-client-language": "en",
      "x-csrf-token": xCsrftoken,
      Host: "api.x.com",
      authorization: authorization,
      Cookie: `lang=en; auth_token=${authToken}; ct0=${ct0}`,
      "Accept-Encoding": "identity",
      Connection: "close",
    },
    secretHeaders: [
      `x-csrf-token: ${xCsrftoken}`,
      `cookie: lang=en; auth_token=${authToken}; ct0=${ct0}`,
      `authorization: ${authorization}`,
    ],
  };
};

export const useTlsnProover = () => {
  const { proofUrl } = useProofContext();
  const [proof, setProof] = useState<any>();
  const [isProoving, setIsProoving] = useState(false);
  const [hasDataForProof, setHasDataForProof] = useState(false);
  const [cookies, setCookies] = useState<browser.Cookies.Cookie[]>([]);
  const [headers, setHeaders] = useState<
    browser.WebRequest.HttpHeadersItemType[]
  >([]);

  const [fotmattedHeaders, setFormattedHeaders] = useState<{
    headers: Record<string, string>;
    secretHeaders: string[];
  }>({
    headers: {},
    secretHeaders: [],
  });

  useEffect(() => {
    const formattedHeaders = formatTlsnHeaders(
      headers,
      cookies,
      hasDataForProof,
    );
    console.log("formattedHeaders", formattedHeaders);
    setFormattedHeaders(formattedHeaders);
  }, [headers, cookies, hasDataForProof]);

  useEffect(() => {
    setHasDataForProof(cookies.length > 0 && headers.length > 0);
  }, [cookies, headers]);

  useEffect(() => {
    browser.webRequest.onResponseStarted.addListener(
      async (details) => {
        if (details.url.includes(proofUrl)) {
          const cookies = await browser.cookies.getAll({ url: details.url });
          setCookies(cookies);
        }
      },
      { urls: ["<all_urls>"] },
    );
    browser.webRequest.onBeforeSendHeaders.addListener(
      (details) => {
        if (details.url.includes(proofUrl)) {
          const headers: browser.WebRequest.HttpHeadersItemType[] = [];
          details.requestHeaders?.forEach((header) => {
            headers.push(header);
          });
          setHeaders(headers);
        }
      },
      { urls: ["<all_urls>"] },
      ["requestHeaders"],
    );
  }, []);

  const prove = useCallback(async () => {
    setIsProoving(true);
    const tlsnProof = await tlsnProve(proofUrl, {
      notaryUrl: import.meta.env.VITE_NOTARY_URL,
      websocketProxyUrl: `${import.meta.env.VITE_WEBSOCKET_PROXY_URL}?token=${new URL(proofUrl).host}`,
      method: "GET",
      headers: fotmattedHeaders.headers,
      secretHeaders: fotmattedHeaders.secretHeaders,
    });
    setProof(tlsnProof);
    setIsProoving(false);
  }, [cookies, headers]);
  return {
    prove,
    proof,
    isProoving,
    hasDataForProof,
  };
};
