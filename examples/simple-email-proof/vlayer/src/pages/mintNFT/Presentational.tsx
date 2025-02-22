export const MintNFT = ({
  currentStep,
  handleProving,
}: {
  currentStep: string;
  handleProving: () => void;
}) => {
  return (
    <>
      <div className="mt-5 flex justify-center">
        <button
          type="button"
          id="nextButton"
          data-testid="connect-wallet-button"
          onClick={handleProving}
        >
          {currentStep ? currentStep : "Mint"}
        </button>
      </div>
    </>
  );
};
