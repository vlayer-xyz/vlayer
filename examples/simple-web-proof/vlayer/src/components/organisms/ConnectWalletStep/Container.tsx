import { useAppKit } from "@reown/appkit/react";
import { ConnectWalletStepPresentational } from "./Presentational";
import { useModal } from "../../../hooks/useModal";
import { useCallback, useEffect } from "react";
import { useNavigate } from "react-router";
import { useExtension } from "../../../hooks/useExtension";
import { useAccount } from "wagmi";

const useConnectWallet = () => {
  const { open: openWallet } = useAppKit();
  const { closeModal, showModal } = useModal();
  const { hasExtensionInstalled } = useExtension();
  const navigate = useNavigate();
  const account = useAccount();

  const isWalletConnected = account.isConnected;

  const next = useCallback(() => {
    if (hasExtensionInstalled) {
      void navigate("/start-proving");
    } else {
      void navigate("/install-extension");
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [hasExtensionInstalled]);

  const connectWallet = async () => {
    await openWallet();
    closeModal();
  };

  useEffect(() => {
    showModal();
    // eslint-disable-next-line react-hooks/exhaustive-deps
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
      connectWallet={() => void connectWallet()}
    />
  );
};
