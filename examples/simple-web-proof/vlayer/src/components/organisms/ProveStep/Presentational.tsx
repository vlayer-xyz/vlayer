export const ProveStepPresentational = ({
  requestWebProof,
  isPending,
  disabled,
  setDisabled,
  errorMsg,
}: {
  requestWebProof: () => void;
  isPending: boolean;
  disabled: boolean;
  setDisabled: (disabled: boolean) => void;
  errorMsg?: string;
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
        {errorMsg && (
          <div
            role="alert"
            className="alert alert-error m-2"
            style={{ width: "300px", height: "48px" }}
          >
            <span>Proving error</span>
          </div>
        )}
      </div>
    </>
  );
};
