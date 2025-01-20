import { useEffect, useRef } from "react";
import { Link } from "react-router";

export const Minting = () => {
  const modalRef = useRef<HTMLDialogElement>(null);

  const handleMint = () => {};

  useEffect(() => {
    modalRef.current?.showModal();
  }, []);

  return (
    <>
      <button className="btn" onClick={() => modalRef.current?.showModal()}>
        Start
      </button>
      <dialog id="my_modal_3" className="modal" ref={modalRef}>
        <div className="modal-box bg-white rounded-2xl ">
          <form method="dialog">
            <Link
              to="/start-proving"
              className="absolute left-3 text-black top-3 text-xs font-normal"
            >
              Back
            </Link>
            <button className="btn btn-sm btn-circle btn-ghost absolute right-3 text-black top-3">✕</button>
          </form>
          <ul className="steps w-full">
            <li className="step step-primary text-black text-xs">Connect Wallet</li>
            <li className="step step-primary text-black text-xs">Get data from X</li>
            <li className="step step-primary text-black text-xs font-bold">Mint NFT</li>
          </ul>
          <h3 className="mt-7 text-center text-black text-3xl font-bold ">
            X NFT
          </h3>
          <p className="py-4 text-gray-500">
            You are all set to mint your unique @satoshi X NFT, a true reflection of your verified identity.
          </p>
          <div className="mt-7 flex justify-center">
            <button
              className="btn w-[188px] px-4 bg-[#915bf8] rounded-lg border-none text-white hover:bg-[#915bf8]/80 hover:text-white"
              onClick={handleMint}
            >
              Start Minting
            </button>
          </div>
        </div>
      </dialog>
    </>
  );
};
