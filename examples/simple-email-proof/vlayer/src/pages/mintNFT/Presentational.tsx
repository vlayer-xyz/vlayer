import { InfoTip } from "./InfoTip";

export const MintNFT = ({
  copyCode,
  setCopyCode,
  emlFile,
  setEmlFile,
  currentStep,
  handleProving,
}: {
  copyCode: boolean;
  setCopyCode: (copyCode: boolean) => void;
  emlFile: File | undefined;
  setEmlFile: (emlFile: File | undefined) => void;
  currentStep: string;
  handleProving: () => void;
}) => {
  return (
    <>
      <div className="w-full">
        {copyCode && (
          <label className="form-control w-full">
            <div className="label">
              <span className="label-text">Copied code</span>
              <span className="label-text-alt">
                or{" "}
                <b
                  className="text-violet-500 cursor-pointer"
                  onClick={() => setCopyCode(false)}
                >
                  Upload file
                </b>
              </span>
            </div>
            <label className="input input-bordered flex items-center gap-2 border-gray-300 text-black bg-white">
              <input
                type="text"
                placeholder="Paste *.eml file content here"
                className="w-full"
              />
            </label>
          </label>
        )}
        {!copyCode && (
          <div className="form-control w-full">
            <div className="label">
              <span className="label-text">Upload eml file</span>
              <span className="label-text-alt">
                or{" "}
                <b
                  className="text-violet-500 cursor-pointer"
                  onClick={() => setCopyCode(true)}
                >
                  Paste content
                </b>
              </span>
            </div>
            <label className="input input-bordered flex items-center gap-2 border-gray-300 text-black bg-white">
              <input
                type="file"
                className="w-full"
                onChange={(e) => setEmlFile(e.target.files?.[0] || undefined)}
              />
            </label>
          </div>
        )}
        <InfoTip />
      </div>
      <div className="mt-5 flex justify-center">
        <button
          type="button"
          id="nextButton"
          data-testid="connect-wallet-button"
          onClick={handleProving}
          disabled={!emlFile}
        >
          {currentStep ? currentStep : "Mint"}
        </button>
      </div>
    </>
  );
};
