import { useEffect, useRef } from "react";
import { Link } from "react-router";

export const WelcomeScreen = () => {
  const modalRef = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    modalRef.current?.showModal();
  }, []);

  return (
    <>
      <button className="btn" onClick={() => modalRef.current?.showModal()}>
        Start
      </button>
      <dialog className="modal" ref={modalRef}>
        <div className="modal-box bg-white rounded-2xl">
          <div className="flex justify-center mb-4">
            <img
              src="/nft-illustration.svg"
              alt="NFT Icon"
              className="w-[282px] h-[156px]"
            />
          </div>
          <h3 className="text-center text-black text-3xl font-bold ">X NFT</h3>
          <p className="text-center font-normal text-lg text-gray-500">
            Mint an NFT with your X (previosuly twitter) account. Only owner of
            the account can mint NFT for specific handle. This example
            demonstrates use of Web Proofs.
          </p>
          <div className="mt-5 flex justify-center">
            <Link
              to="/connect-wallet"
              className="btn w-[188px] px-4 bg-[#915bf8] rounded-lg border-none text-white hover:bg-[#915bf8]/80 hover:text-white"
            >
              Start
            </Link>
          </div>
        </div>
      </dialog>
    </>
  );
};
