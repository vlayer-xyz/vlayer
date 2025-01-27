import { FormEvent, FC } from "react";

interface EmlFormProps {
  isSubmitting: boolean;
  currentStep: string;
  errorMsg: string;
  successMsg: string;
  handleSubmit: (e: FormEvent<HTMLFormElement>) => Promise<void> | void;
}

const EmlForm: FC<EmlFormProps> = ({
  isSubmitting,
  currentStep,
  errorMsg,
  successMsg,
  handleSubmit,
}) => {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-900">
      <div className="w-full max-w-md m-6">
        <h1 className="text-center mb-3 text-xl">Claim your Email NFT badge</h1>
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

          {errorMsg && (
            <p className="text-block text-center text-red-400 mt-5">
              Error: {errorMsg}
            </p>
          )}
          {successMsg && (
            <p
              dangerouslySetInnerHTML={{ __html: successMsg }}
              className="text-block text-center text-green-400 mt-5"
            />
          )}
        </form>
        <p className="text-center text-gray-400 text-sm mt-5">
          <a
            href="#showModal"
            onClick={() =>
              (
                document.getElementById("my_modal_1") as HTMLDialogElement
              )?.showModal()
            }
          >
            How to get .eml file?
          </a>
        </p>
        <dialog id="my_modal_1" className="modal">
          <div className="modal-box">
            <h3 className="font-bold text-lg">Getting .eml files from inbox</h3>
            <div className="py-4">
              <b>Gmail</b>
              <ol className="list-decimal list-inside mt-2">
                <li>Open the email you want to save</li>
                <li>
                  Click the three-dot menu in the top-right corner of the email
                </li>
                <li>Select Download message</li>
              </ol>
            </div>
            <div className="py-4">
              <b>Outlook / Thunderbird</b>
              <ol className="list-decimal list-inside mt-2">
                <li>Open the email you want to save</li>
                <li>Click on the File menu.</li>
                <li>Select "Save As".</li>
              </ol>
            </div>
            <p className="py-4 text-center">
              More info in{" "}
              <a
                href="https://book.vlayer.xyz/features/email.html"
                className="text-violet-500"
              >
                vlayer book
              </a>
            </p>
            <div className="modal-action">
              <form method="dialog">
                <button className="btn">Close</button>
              </form>
            </div>
          </div>
        </dialog>
      </div>
    </div>
  );
};

export default EmlForm;
