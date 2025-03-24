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
      <div className="mt-7 flex justify-center flex-col items-center">
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
