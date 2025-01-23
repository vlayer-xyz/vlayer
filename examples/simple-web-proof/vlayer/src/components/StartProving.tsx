export const StartProving = ({
  requestWebProof,
  isPending,
  disabled,
  setDisabled,
  error,
}: {
  requestWebProof: () => void;
  isPending: boolean;
  disabled: boolean;
  setDisabled: (disabled: boolean) => void;
  error?: string;
}) => {
  return (
    <>
      <ul className="steps w-full">
        <li className="step step-primary text-black text-xs font-bold">
          Get data from X
        </li>
        <li className="step text-black text-xs">Verify</li>
      </ul>
      <h3 className="header">Proof of X Followers</h3>
      <p className="desc">
        Open our browser extension and follow instructions in order to produce
        the Proof of X account followers.
      </p>
      <div className="mt-7 flex justify-center">
        <button
          disabled={disabled}
          id="nextButton"
          onClick={() => {
            console.log("open extension");
            requestWebProof();
            setDisabled(true);
          }}
        >
          {isPending ? "Proving in progress..." : "Open Extension"}
        </button>
      </div>
      {error && <p className="text-red-500 w-full block">{error}</p>}
    </>
  );
};
