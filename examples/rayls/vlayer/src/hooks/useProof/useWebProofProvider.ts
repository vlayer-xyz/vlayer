import { createExtensionWebProofProvider } from "@vlayer/sdk/web_proof";
import { useMemo } from "react";

console.log(import.meta.env.VITE_NOTARY_URL, import.meta.env.VITE_WS_PROXY_URL);
export const useWebProofProvider = () =>
  useMemo(
    () =>
      createExtensionWebProofProvider({
        notaryUrl: import.meta.env.VITE_NOTARY_URL,
        wsProxyUrl: import.meta.env.VITE_WS_PROXY_URL,
      }),
    [],
  );
