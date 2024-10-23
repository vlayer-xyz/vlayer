'use client';

import {PrivyProvider} from '@privy-io/react-auth';
import { optimismSepolia } from 'viem/chains';

export default function Providers({children}: {children: React.ReactNode}) {
  return (
    <PrivyProvider
      appId={process.env.NEXT_PUBLIC_PRIVY_IO ?? "insert_valid_key"}
      config={{
        defaultChain: optimismSepolia, 
        supportedChains: [optimismSepolia] 
      }}
    >
      {children}
    </PrivyProvider>
  );
}