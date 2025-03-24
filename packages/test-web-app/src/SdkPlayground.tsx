import {
  createExtensionWebProofProvider,
  createVlayerClient,
} from "@vlayer/sdk";

import React, { useEffect, useState } from "react";

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
  const [provider, setProvider] =
    useState<ReturnType<typeof createExtensionWebProofProvider>>();

  useEffect(() => {
    const provider = createExtensionWebProofProvider({});
    setProvider(provider);
    if (!window._vlayer && provider) {
      window._vlayer = {
        extensionWebProofProvider: provider,
        vlayerClient: createVlayerClient({ webProofProvider: provider }),
      };
    }
  }, [!!provider]);
  return <></>;
};

export default SdkPlayground;
