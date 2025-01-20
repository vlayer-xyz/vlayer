import {
  useAppKit,
  useAppKitAccount,
  useDisconnect,
} from "@reown/appkit/react";
import { useEffect, useRef } from "react";
import { Link, useNavigate } from "react-router";

export const ConnectWallet = () => {
  const navigate = useNavigate();
  const { open, close } = useAppKit();
  const { isConnected, address } = useAppKitAccount();
  const { disconnect } = useDisconnect();
  const modalRef = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    modalRef.current?.showModal();
  }, []);

  useEffect(() => {
    if (isConnected) {
      modalRef.current?.showModal();
      // navigate("/start-proving");
    }
  }, [isConnected]);

  return (
    <>
      <button 
        className="btn"
        onClick={() => {
          modalRef.current?.showModal();
          close();
        }}
      >
        Start
      </button>
      <dialog id="my_modal_3" className="modal" ref={modalRef}>
        <div className="modal-box bg-white rounded-2xl ">
          <form method="dialog">
            <Link
              to="/"
              className="absolute left-3 text-black top-3 text-xs font-normal"
            >
              Back
            </Link>
            <button className="btn btn-sm btn-circle btn-ghost absolute right-3 text-black top-3">✕</button>
          </form>
          <ul className="steps w-full">
            <li className="step step-primary text-black text-xs font-bold">
              Connect Wallet
            </li>
            <li className="step text-black text-xs">Get data from X</li>
            <li className="step text-black text-xs">Mint NFT</li>
          </ul>
          <h3 className="mt-7 text-center text-black text-3xl font-bold ">
            X NFT
          </h3>
          <p className="py-4 text-gray-500">
            To proceed to the next step, please connect your wallet now by clicking the button below.
          </p>
          <div className="mt-7 flex justify-center">
            {isConnected ? (
              <button
                onClick={() => {
                  navigate("/start-proving");
                }}
                className="btn w-[188px] px-4 bg-[#915bf8] rounded-lg border-none text-white hover:bg-[#915bf8]/80 hover:text-white"
              >
                Start Minting
              </button>
            ) : (
              <button
                onClick={() => {
                  open();
                  modalRef.current?.close();
                }}
                className="btn w-[188px] px-4 bg-[#915bf8] rounded-lg border-none text-white hover:bg-[#915bf8]/80 hover:text-white"
              >
                Connect Wallet
              </button>
            )}
          </div>
          {isConnected && address && (
            <p className="mt-3 text-black text-xs font-bold block w-full">
              Connected as <br />
              {address} <br/>
              <button className="btn btn-sm btn-error" onClick={disconnect}>
                disconnect
              </button>
            </p>
          )}
        </div>
      </dialog>
    </>
  );
};
