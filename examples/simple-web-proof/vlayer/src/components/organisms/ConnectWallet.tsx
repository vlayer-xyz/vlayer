import { useAppKit } from "@reown/appkit/react";
import { useEffect } from "react";
import { useNavigate } from "react-router";
import { useModal } from "../../hooks/useModal";
import { useAppKitAccount } from "@reown/appkit/react";

export const ConnectWallet = () => {
  const navigate = useNavigate();
  const { open: openWallet } = useAppKit();
  const { isConnected: isWalletConnected } = useAppKitAccount();
  const { closeModal, showModal } = useModal();

  const navigateToStartProving = () => {
    navigate("/proof/start-proving");
  };

  const connectWallet = () => {
    openWallet();
    closeModal();
  };

  useEffect(() => {
    showModal();
  }, [isWalletConnected]);

  return (
    <>
      <h3 className="header">X NFT</h3>
      <p className="desc">
        To proceed to the next step, please connect your wallet now by clicking
        the button below.
      </p>
      <div className="mt-7 flex justify-center">
        {isWalletConnected ? (
          <button onClick={navigateToStartProving} id="nextButton">
            Start Proving
          </button>
        ) : (
          <button onClick={connectWallet} id="nextButton">
            Connect Wallet
          </button>
        )}
      </div>
    </>
  );
};
