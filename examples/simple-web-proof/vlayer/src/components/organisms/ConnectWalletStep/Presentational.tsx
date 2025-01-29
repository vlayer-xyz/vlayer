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
