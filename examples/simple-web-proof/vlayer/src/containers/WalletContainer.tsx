import { useAppKit, useAppKitAccount } from "@reown/appkit/react";
import { Modal } from "../components/Modal";
import { ConnectWallet } from "../components/ConnectWallet";

export const WalletContainer = () => {
  const { open } = useAppKit();
  const { isConnected } = useAppKitAccount();

  return (
    <Modal backUrl="/">
      <ConnectWallet isWalletConnected={isConnected} openWallet={open} />
    </Modal>
  );
};
