import { useAppKit, useAppKitAccount } from "@reown/appkit/react";
import { ConnectWalletStepPresentational } from "./Presentational";
import { useModal } from "../../../hooks/useModal";
import { useEffect } from "react";
import { useNavigate } from "react-router";

const useConnectWallet = () => {
  const { open: openWallet } = useAppKit();
  const { closeModal, showModal } = useModal();

  const navigate = useNavigate();
  const { isConnected: isWalletConnected } = useAppKitAccount();

  const navigateToStartProving = () => {
    navigate("/start-proving");
  };

  const connectWallet = () => {
    openWallet();
    closeModal();
  };

  useEffect(() => {
    showModal();
  }, [isWalletConnected]);

  return {
    navigateToStartProving,
    connectWallet,
    isWalletConnected,
  };
};

export const ConnectWalletStep = () => {
  const { isWalletConnected, navigateToStartProving, connectWallet } =
    useConnectWallet();
  return (
    <ConnectWalletStepPresentational
      isWalletConnected={isWalletConnected}
      navigateToStartProving={navigateToStartProving}
      connectWallet={connectWallet}
    />
  );
};
