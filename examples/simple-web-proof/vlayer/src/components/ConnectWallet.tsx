import { useEffect } from "react";
import { useNavigate } from "react-router";

export const ConnectWallet = ({
  showModal,
  closeModal,
  isWalletConnected,
  openWallet,
}: {
  showModal?: () => void;
  closeModal?: () => void;
  isWalletConnected: boolean;
  openWallet: () => void;
}) => {
  const navigate = useNavigate();

  useEffect(() => {
    showModal?.();
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
          <button
            onClick={() => {
              navigate("/proof/start-proving");
            }}
            id="nextButton"
          >
            Start Proving
          </button>
        ) : (
          <button
            onClick={() => {
              openWallet();
              closeModal?.();
            }}
            id="nextButton"
          >
            Connect Wallet
          </button>
        )}
      </div>
    </>
  );
};
