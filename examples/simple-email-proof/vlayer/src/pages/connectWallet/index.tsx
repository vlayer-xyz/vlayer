import { useConnectWallet } from "./useConnectWallet";

export const ConnectWallet = () => {
  const { connectWallet } = useConnectWallet();

  return (
    <div className="mt-5 flex justify-center">
      <button
        onClick={connectWallet}
        id="nextButton"
        data-testid="connect-wallet-button"
      >
        Connect Wallet
      </button>
    </div>
  );
};
