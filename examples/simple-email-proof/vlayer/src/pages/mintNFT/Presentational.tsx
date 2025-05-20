export const MintNFT = ({
  currentStep,
  handleProving,
  verificationError,
}: {
  currentStep: string;
  handleProving: () => void;
  verificationError: string | null;
}) => {
  return (
    <>
      <div className="mt-5 flex justify-center">
        <button
          disabled={currentStep != "Mint"}
          type="button"
          id="nextButton"
          data-testid="mint-nft-button"
          onClick={handleProving}
        >
          {currentStep}
        </button>
      </div>
      {verificationError && (
        <div className="mt-5 flex justify-center">
          <p className="text-red-500">{verificationError}</p>
        </div>
      )}
    </>
  );
};
