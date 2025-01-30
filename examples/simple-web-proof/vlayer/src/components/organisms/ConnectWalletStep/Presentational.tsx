export const ConnectWalletStepPresentational = ({
  isWalletConnected,
  next,
  connectWallet,
}: {
  isWalletConnected: boolean;
  next: () => void;
  connectWallet: () => void;
}) => {
  return (
    <>
      <div className="flex justify-center modal-action">
        {isWalletConnected ? (
          <button onClick={next} id="nextButton">
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
