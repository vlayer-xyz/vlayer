export const ProveStepPresentational = ({
  requestWebProof,
  isPending,
  disabled,
  setDisabled,
}: {
  requestWebProof: () => void;
  isPending: boolean;
  disabled: boolean;
  setDisabled: (disabled: boolean) => void;
}) => {
  return (
    <>
      <h3 className="header">X NFT</h3>
      <p className="desc">
        Open vlayer browser extension and follow instructions in order to
        produce the Proof of X account ownership.
      </p>
      <div className="mt-7 flex justify-center">
        <button
          disabled={disabled}
          id="nextButton"
          onClick={() => {
            requestWebProof();
            setDisabled(true);
          }}
        >
          {isPending ? "Proving in progress..." : "Open Extension"}
        </button>
      </div>
    </>
  );
};
