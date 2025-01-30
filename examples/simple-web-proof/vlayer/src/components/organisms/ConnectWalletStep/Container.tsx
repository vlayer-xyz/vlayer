import { useAppKit, useAppKitAccount } from "@reown/appkit/react";
import { ConnectWalletStepPresentational } from "./Presentational";
import { useModal } from "../../../hooks/useModal";
import { useCallback, useEffect } from "react";
import { useNavigate } from "react-router";
import { useExtension } from "../../../hooks/useExtension";

const useConnectWallet = () => {
  const { open: openWallet } = useAppKit();
  const { closeModal, showModal } = useModal();
  const { hasExtensionInstalled } = useExtension();
  const navigate = useNavigate();
  const { isConnected: isWalletConnected } = useAppKitAccount();

  const next = useCallback(() => {
    if (hasExtensionInstalled) {
      navigate("/start-proving");
    } else {
      navigate("/install-extension");
    }
  }, [hasExtensionInstalled]);

  const connectWallet = () => {
    openWallet();
    closeModal();
  };

  useEffect(() => {
    showModal();
  }, [isWalletConnected]);

  return {
    next,
    connectWallet,
    isWalletConnected,
  };
};

export const ConnectWalletStep = () => {
  const { isWalletConnected, next, connectWallet } = useConnectWallet();
  return (
    <ConnectWalletStepPresentational
      isWalletConnected={isWalletConnected}
      next={next}
      connectWallet={connectWallet}
    />
  );
};
