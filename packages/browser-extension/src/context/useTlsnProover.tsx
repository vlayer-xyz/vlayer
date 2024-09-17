import { prove as tlsnProve, verify as tlsnVerify, verify } from "tlsn-js";
import browser from "webextension-polyfill";
import { useProofContext } from "./useProofContext";
import React, {
  useContext,
  createContext,
  useCallback,
  useEffect,
  useState,
  PropsWithChildren,
} from "react";
import { formatTlsnHeaders } from "../lib/formatTlsnHeaders";

const TlsnProofContext = createContext({
  prove: () => {},
  proof: null,
  isProoving: false,
  hasDataForProof: false,
});

export const TlsnProofContextProvider = ({ children }: PropsWithChildren) => {
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
    try {
      const tlsnProof = await tlsnProve(proofUrl, {
        notaryUrl: import.meta.env.VITE_NOTARY_URL,
        websocketProxyUrl: `${import.meta.env.VITE_WEBSOCKET_PROXY_URL}?token=${new URL(proofUrl).host}`,
        method: "GET",
        headers: fotmattedHeaders.headers,
        secretHeaders: fotmattedHeaders.secretHeaders,
      });
      const verifiedProof = await verify(tlsnProof);
      setProof(verifiedProof);
      setIsProoving(false);
    } catch (e) {
      console.error("error in tlsnotary", e);
      setIsProoving(false);
    }
  }, [cookies, headers]);

  return (
    <TlsnProofContext.Provider
      value={{ prove, proof, isProoving, hasDataForProof }}
    >
      {children}
    </TlsnProofContext.Provider>
  );
};

export const useTlsnProover = () => {
  return useContext(TlsnProofContext);
};
