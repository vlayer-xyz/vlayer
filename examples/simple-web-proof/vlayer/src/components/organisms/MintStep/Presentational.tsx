export const MintStepPresentational = ({
  handleMint,
  isMinting,
  errorMsg,
}: {
  handleMint: () => void;
  isMinting: boolean;
  errorMsg?: string;
}) => {
  return (
    <>
      <div className="mt-7 flex justify-center flex-col items-center">
        <button disabled={isMinting} id="nextButton" onClick={handleMint}>
          {isMinting ? "Minting..." : "Start Minting"}
        </button>
        {errorMsg && (
          <div
            role="alert"
            className="alert alert-error m-2"
            style={{ width: "300px" }}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-6 w-6 shrink-0 stroke-current"
              fill="none"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth="2"
                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
            <span>Already minted </span>
          </div>
        )}
      </div>
    </>
  );
};
