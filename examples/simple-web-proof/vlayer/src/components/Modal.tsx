import { cloneElement, isValidElement, useEffect, useRef } from "react";
import { Link } from "react-router";

export const Modal = ({
  backUrl,
  children,
}: {
  backUrl?: string;
  children: React.ReactNode;
}) => {
  const modalRef = useRef<HTMLDialogElement>(null);

  const showModal = () => {
    modalRef.current?.showModal();
  };

  const closeModal = () => {
    modalRef.current?.close();
  };

  useEffect(() => {
    showModal();
  }, []);

  const childrenWithProps = isValidElement(children)
    ? cloneElement(
        children as React.ReactElement<{
          showModal?: () => void;
          closeModal?: () => void;
        }>,
        {
          showModal,
          closeModal,
        },
      )
    : children;

  return (
    <>
      <button className="btn" onClick={showModal}>
        Start
      </button>
      <dialog className="modal" ref={modalRef}>
        <div className="modal-box bg-white rounded-2xl">
          {backUrl && (
            <form method="dialog">
              <Link
                to={backUrl}
                className="absolute left-3 text-black top-3 text-xs font-normal"
              >
                Back
              </Link>
            </form>
          )}
          {childrenWithProps}
        </div>
      </dialog>
    </>
  );
};
