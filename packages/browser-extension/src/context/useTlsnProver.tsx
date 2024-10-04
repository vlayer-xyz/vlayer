import { prove as tlsnProve } from "tlsn-js";
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
import {
  ExtensionMessage,
  WebProverSessionConfig,
} from "@vlayer/web-proof-commons/constants/message";

const TlsnProofContext = createContext({
  prove: () => {},
  proof: null as object | null,
  isProving: false,
  hasDataForProof: false,
});

export const TlsnProofContextProvider = ({ children }: PropsWithChildren) => {
  const { proofUrl } = useProofContext();
  const [proof, setProof] = useState<object | null>(null);
  const [isProving, setIsProving] = useState(false);
  const [hasDataForProof, setHasDataForProof] = useState(false);
  const [cookies, setCookies] = useState<browser.Cookies.Cookie[]>([]);
  const [headers, setHeaders] = useState<
    browser.WebRequest.HttpHeadersItemType[]
  >([]);

  const [formattedHeaders, setFormattedHeaders] = useState<{
    headers: Record<string, string>;
    secretHeaders: string[];
  } | null>({
    headers: {},
    secretHeaders: [],
  });

  useEffect(() => {
    setFormattedHeaders(formatTlsnHeaders(headers, cookies));
  }, [headers, cookies]);

  useEffect(() => {
    setHasDataForProof(!!formattedHeaders);
  }, [formattedHeaders]);

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
    setIsProving(true);

    const webProverSessionConfig = (
      await browser.storage.local.get("webProverSessionConfig")
    ).webProverSessionConfig as WebProverSessionConfig;

    try {
      console.log(
        `Calling tlsnProve with ${proofUrl}, ${webProverSessionConfig.notaryUrl}, ${webProverSessionConfig.wsProxyUrl}`,
      );
      const tlsnProof = await tlsnProve(proofUrl, {
        notaryUrl: webProverSessionConfig.notaryUrl,
        websocketProxyUrl: webProverSessionConfig.wsProxyUrl,
        method: "GET",
        headers: formattedHeaders?.headers,
        secretHeaders: formattedHeaders?.secretHeaders,
      });
      // this is temporary verification call
      // when we wil connect vlayer contracts we will transfer this back to the SDK

      browser.runtime.sendMessage({
        type: ExtensionMessage.ProofDone,
        proof: tlsnProof,
      });
      setProof(proof);
      setIsProving(false);
    } catch (e) {
      console.error("error in tlsnotary", e);

      browser.runtime.sendMessage({
        type: ExtensionMessage.ProofError,
        error: e,
      });

      setIsProving(false);
    }
  }, [formattedHeaders]);

  return (
    <TlsnProofContext.Provider
      value={{ prove, proof, isProving, hasDataForProof }}
    >
      {children}
    </TlsnProofContext.Provider>
  );
};

export const useTlsnProver = () => {
  return useContext(TlsnProofContext);
};
