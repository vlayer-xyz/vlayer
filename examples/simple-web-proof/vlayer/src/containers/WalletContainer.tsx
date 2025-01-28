import { useAppKit, useAppKitAccount } from "@reown/appkit/react";
import { ConnectWallet } from "../components/ConnectWallet";

export const WalletContainer = () => {
  const { open } = useAppKit();
  const { isConnected } = useAppKitAccount();

  return <ConnectWallet isWalletConnected={isConnected} openWallet={open} />;
};
