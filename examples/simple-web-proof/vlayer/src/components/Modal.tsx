import { cloneElement, isValidElement, useEffect, useRef } from "react";
import { Link } from "react-router";
import { motion } from "motion/react";

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
    <dialog className="modal" ref={modalRef}>
      <motion.div
        className="modal-box bg-white rounded-2xl"
        initial={{ opacity: 0, scale: 0.1 }}
        animate={{ opacity: 1, scale: 1 }}
        exit={{ opacity: 0, scale: 0.1 }}
        transition={{ ease: "easeOut", duration: 0.5 }}
      >
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
      </motion.div>
    </dialog>
  );
};
