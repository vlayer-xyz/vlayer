import React from "react";
import { WagmiProvider } from "wagmi";
import { ProofProvider } from "@vlayer/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { wagmiConfig, proverConfig } from "./config";

const queryClient = new QueryClient();

const Providers = ({ children }: { children: React.ReactNode }) => {
  return (
    <WagmiProvider config={wagmiConfig}>
      <QueryClientProvider client={queryClient}>
        <ProofProvider config={proverConfig}>{children}</ProofProvider>
      </QueryClientProvider>
    </WagmiProvider>
  );
};

export default Providers;
