import { useEffect, useRef } from "react";
import { Link } from "react-router";

export const Success = () => {
  const modalRef = useRef<HTMLDialogElement>(null);

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
          <div className="flex justify-center">
            <img
              src="/success-illustration.svg"
              alt="Success Icon"
              className="w-[282px] h-[155px]"
            />
          </div>
          <h3 className="mt-7 header">Success</h3>
          <p className="py-4 text-gray-500">
            @satoshi NFT was minted to 0x1234...abcd
          </p>
          <div className="mt-7 flex justify-center">
            <Link to="/" id="nextButton">
              Start again
            </Link>
          </div>
        </div>
      </dialog>
    </>
  );
};
