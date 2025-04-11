import {
  createExtensionWebProofProvider,
  createVlayerClient,
} from "@vlayer/sdk";

import React, { useEffect } from "react";

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
    const provider = createExtensionWebProofProvider();
    window._vlayer = {
      extensionWebProofProvider: provider,
      vlayerClient: createVlayerClient({
        webProofProvider: provider,
      }),
    };
  }, []);
  return <></>;
};

export default SdkPlayground;
