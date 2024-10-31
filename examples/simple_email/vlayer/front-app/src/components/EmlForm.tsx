interface EmlFormProps {
  isSubmitting: boolean;
  currentStep: string;
  errorMsg: string;
  handleSubmit: (event: React.FormEvent<HTMLFormElement>) => void;
}

const EmlForm: React.FC<EmlFormProps> = ({
  isSubmitting,
  currentStep,
  errorMsg,
  handleSubmit,
}) => {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-900">
      <div className="w-full max-w-md m-6">
        <h1 className="text-center mb-3 text-xl">
          Claim your @vlayer.xyz NFT badge
        </h1>
        <form
          onSubmit={handleSubmit}
          className="rounded-lg px-8 pt-6 pb-8 mb-4 border border-violet-600"
        >
          <div className="mb-6">
            <label
              className="block text-gray-200 text-sm font-bold mb-2"
              htmlFor="emlFile"
            >
              EML File Upload
            </label>
            <input
              id="emlFile"
              name="emlFile"
              type="file"
              accept=".eml"
              className="file-input file-input-bordered file-input-primary w-full"
              required
            />
          </div>

          <div className="flex items-center justify-center">
            <button type="submit" className="btn btn-primary w-full">
              {isSubmitting ? currentStep : "Connect & Claim NFT"}
            </button>
          </div>

          <p className="text-block text-center text-red-400 mt-5">{errorMsg}</p>
        </form>
      </div>
    </div>
  );
};

export default EmlForm;
