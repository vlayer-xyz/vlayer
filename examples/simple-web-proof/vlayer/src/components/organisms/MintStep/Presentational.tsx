export const MintStepPresentational = ({
  mintedHandle,
  handleMint,
  isMinting,
  errorMsg,
}: {
  mintedHandle: string;
  handleMint: () => void;
  isMinting: boolean;
  errorMsg?: string;
}) => {
  return (
    <>
      <ul className="steps w-full">
        <li className="step step-primary text-black text-xs">Connect Wallet</li>
        <li className="step step-primary text-black text-xs">
          Get data from X
        </li>
        <li className="step step-primary text-black text-xs font-bold">
          Mint NFT
        </li>
      </ul>
      <h3 className="header">X NFT</h3>
      <p className="py-4 text-gray-500">
        You are all set to mint your unique @{mintedHandle} X NFT, a true
        reflection of your verified identity.
      </p>
      <div className="mt-7 flex justify-center">
        <button disabled={isMinting} id="nextButton" onClick={handleMint}>
          {isMinting ? "Minting..." : "Start Minting"}
        </button>
      </div>
      {errorMsg && <p className="text-red-500 mt-5">Error: {errorMsg}</p>}
    </>
  );
};
