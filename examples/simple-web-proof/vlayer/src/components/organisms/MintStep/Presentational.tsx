export const MintStepPresentational = ({
  handleMint,
  isMinting,
}: {
  handleMint: () => void;
  isMinting: boolean;
}) => {
  return (
    <>
      <div className="mt-7 flex justify-center flex-col items-center">
        <button disabled={isMinting} id="nextButton" onClick={handleMint}>
          {isMinting ? "Minting..." : "Start Minting"}
        </button>
      </div>
    </>
  );
};
