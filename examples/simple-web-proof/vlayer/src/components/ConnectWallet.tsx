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
      <ul className="steps w-full">
        <li className="step step-primary text-black text-xs font-bold">
          Connect Wallet
        </li>
        <li className="step text-black text-xs">Get data from X</li>
        <li className="step text-black text-xs">Mint NFT</li>
      </ul>
      <h3 className="header">X NFT</h3>
      <p className="desc">
        To proceed to the next step, please connect your wallet now by clicking
        the button below.
      </p>
      <div className="mt-7 flex justify-center">
        {isWalletConnected ? (
          <button
            onClick={() => {
              navigate("/start-proving");
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
