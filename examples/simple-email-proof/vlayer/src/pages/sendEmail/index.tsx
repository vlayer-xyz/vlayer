import { Link } from "react-router";
import { DocumentDuplicateIcon } from "@heroicons/react/24/outline";

export const SendEmail = () => {
  return (
    <>
      <div className="w-full">
        <label className="form-control w-full">
          <div className="label">
            <span className="label-text">Subject</span>
          </div>
          <label className="input input-bordered flex items-center gap-2 border-gray-300 text-black bg-white">
            <input
              type="text"
              value="Mint my domain NFT at address: 0x1234...abcd"
              className="w-full"
            />
            <span className="label-text-alt">
              <DocumentDuplicateIcon className="w-4 h-4" />
            </span>
          </label>
        </label>
        <label className="form-control w-full">
          <div className="label">
            <span className="label-text">To</span>
          </div>
          <label className="input input-bordered flex items-center gap-2 border-gray-300 text-black bg-white">
            <input type="text" value="nft@vlayer.xyz" className="w-full" />
            <span className="label-text-alt">
              <DocumentDuplicateIcon className="w-4 h-4" />
            </span>
          </label>
        </label>
      </div>
      <div className="mt-5 flex justify-center">
        <Link
          to="/mint-nft"
          id="nextButton"
          data-testid="connect-wallet-button"
        >
          Next
        </Link>
      </div>
    </>
  );
};
