import { vlayerPovingExtensionId } from "../../utils";

export const InstallExtension = ({ error }: { error?: string }) => {
  return (
    <>
      <ul className="steps w-full">
        <li className="step step-primary text-black text-xs">Connect Wallet</li>
        <li className="step step-primary text-black text-xs font-bold">
          Get data from X
        </li>
        <li className="step text-black text-xs">Mint NFT</li>
      </ul>
      <h3 className="header">X NFT</h3>
      <p className="desc">
        Open vlayer browser extension and follow instructions in order to
        produce the Proof of X account ownership.
      </p>
      <div className="mt-7 flex justify-center">
        <button
          id="nextButton"
          onClick={() => {
            window.open(
              `https://chromewebstore.google.com/detail/vlayer/${vlayerPovingExtensionId}/reviews`,
              "_blank",
            );
          }}
        >
          Install Extension
        </button>
      </div>
      {error && <p className="text-red-400 w-full block mt-3">{error}</p>}
    </>
  );
};
