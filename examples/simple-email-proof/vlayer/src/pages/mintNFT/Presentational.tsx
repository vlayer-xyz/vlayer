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
          data-testid="mint-nft-button"
          onClick={handleProving}
        >
          {currentStep}
        </button>
      </div>
    </>
  );
};
