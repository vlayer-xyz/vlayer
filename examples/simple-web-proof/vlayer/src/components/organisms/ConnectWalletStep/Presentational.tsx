export const ConnectWalletStepPresentational = ({
  isWalletConnected,
  navigateToStartProving,
  connectWallet,
}: {
  isWalletConnected: boolean;
  navigateToStartProving: () => void;
  connectWallet: () => void;
}) => {
  return (
    <>
      <div className="flex justify-center modal-action">
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
