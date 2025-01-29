import { useAppKit, useAppKitAccount } from "@reown/appkit/react";
import { ConnectWallet } from "../components/ConnectWallet";
import { useModal } from "../hooks/useModal";

export const WalletContainer = () => {
  const { open } = useAppKit();
  const { isConnected } = useAppKitAccount();
  const { closeModal, showModal } = useModal();

  return (
    <ConnectWallet
      isWalletConnected={isConnected}
      openWallet={open}
      showModal={showModal}
      closeModal={closeModal}
    />
  );
};
