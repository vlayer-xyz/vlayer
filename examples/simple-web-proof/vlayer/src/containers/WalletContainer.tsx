import { useAppKit, useAppKitAccount } from "@reown/appkit/react";
import { ConnectWallet } from "../components/ConnectWallet";
import { useOutletContext } from "react-router";

export const WalletContainer = () => {
  const { open } = useAppKit();
  const { isConnected } = useAppKitAccount();
  const { closeModal, showModal } = useOutletContext<{
    closeModal: () => void;
    showModal: () => void;
  }>();
  return (
    <ConnectWallet
      isWalletConnected={isConnected}
      openWallet={open}
      showModal={showModal}
      closeModal={closeModal}
    />
  );
};
