import { prove as tlsnProve } from "tlsn-js";
import browser from "webextension-polyfill";
import { useProofContext } from "./useProofContext";
import React, {
  createContext,
  PropsWithChildren,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";
import { formatTlsnHeaders } from "lib/formatTlsnHeaders";
import { ExtensionMessageType } from "../web-proof-commons";
import { WebProverSessionContextManager } from "../state/webProverSessionContext";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";

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

  // TODO: replaee above mechanism. This is new mechanism, added instead of replacing to keep working

  const prove = useCallback(async () => {
    setIsProving(true);

    // TODO useProofContext() to get webProverSessionConfig
    const webProverSessionConfig =
      await WebProverSessionContextManager.instance.getWebProverSessionConfig();

    try {
      const tlsnProof = await tlsnProve(proofUrl, {
        notaryUrl: webProverSessionConfig.notaryUrl,
        websocketProxyUrl: webProverSessionConfig.wsProxyUrl,
        method: "GET",
        headers: formattedHeaders?.headers,
        secretHeaders: formattedHeaders?.secretHeaders,
      });
      // this is temporary verification call
      // when we wil connect vlayer contracts we will transfer this back to the SDK

      sendMessageToServiceWorker({
        type: ExtensionMessageType.ProofDone,
        proof: tlsnProof,
      });
      setProof(proof);
      setIsProving(false);
    } catch (e: unknown) {
      console.error("error in tlsnotary", e);

      sendMessageToServiceWorker({
        type: ExtensionMessageType.ProofError,
        error: e instanceof Error ? e.message : String(e),
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
