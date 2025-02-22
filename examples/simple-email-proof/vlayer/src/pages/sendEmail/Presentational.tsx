import { Link } from "react-router";
import { DocumentDuplicateIcon } from "@heroicons/react/24/outline";
import { Toast } from "react-daisyui";

export const SendEmail = ({
  subject,
  uniqueEmail,
}: {
  subject: string;
  uniqueEmail: string;
}) => {
  return (
    <>
      <div className="w-full">
        <label className="form-control w-full">
          <div className="label">
            <span className="label-text">To</span>
          </div>
          <label className="input input-bordered flex items-center gap-2 border-gray-300 text-black bg-white">
            <input
              type="text"
              value={uniqueEmail}
              className="w-full"
              readOnly
            />
            <span
              className="label-text-alt"
              onClick={() => {
                navigator.clipboard.writeText(uniqueEmail);
              }}
            >
              <DocumentDuplicateIcon className="w-4 h-4" />
            </span>
          </label>
        </label>
        <label className="form-control w-full">
          <div className="label">
            <span className="label-text">Subject</span>
          </div>
          <label className="input input-bordered flex items-center gap-2 border-gray-300 text-black bg-white">
            <input type="text" value={subject} className="w-full" readOnly />
            <span
              className="label-text-alt"
              onClick={() => {
                navigator.clipboard.writeText(subject);
              }}
            >
              <DocumentDuplicateIcon className="w-4 h-4" />
            </span>
          </label>
        </label>
      </div>
      <div className="mt-5 flex justify-center">
        <Link
          to={`/collect-email?uniqueEmail=${uniqueEmail}`}
          id="nextButton"
          data-testid="connect-wallet-button"
        >
          Next
        </Link>
      </div>
    </>
  );
};
