import {
  createExtensionWebProofProvider,
  createVlayerClient,
} from "@vlayer/sdk";

import React, { useEffect } from "react";

const TOKEN = import.meta.env.VITE_VLAYER_API_TOKEN;

declare global {
  interface Window {
    _vlayer: {
      extensionWebProofProvider: ReturnType<
        typeof createExtensionWebProofProvider
      >;
      vlayerClient: ReturnType<typeof createVlayerClient>;
    };
  }
}

const SdkPlayground = () => {
  useEffect(() => {
    const provider = createExtensionWebProofProvider({ token: TOKEN });
    window._vlayer = {
      extensionWebProofProvider: provider,
      vlayerClient: createVlayerClient({
        webProofProvider: provider,
        token: TOKEN,
      }),
    };
  }, []);
  return <></>;
};

export default SdkPlayground;
