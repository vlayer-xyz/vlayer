import { createContext, useCallback, useEffect, useRef } from "react";
import { Link } from "react-router";
import { motion } from "motion/react";

export const modalContext = createContext({
  showModal: () => {},
  closeModal: () => {},
});

export const Modal = ({
  backUrl,
  children,
}: {
  backUrl?: string;
  children: React.ReactNode;
}) => {
  const modalRef = useRef<HTMLDialogElement>(null);

  const showModal = useCallback(() => {
    modalRef.current?.showModal();
  }, [modalRef]);

  const closeModal = useCallback(() => {
    modalRef.current?.close();
  }, [modalRef]);

  useEffect(() => {
    showModal();
  }, [showModal]);

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
        <modalContext.Provider value={{ showModal, closeModal }}>
          {children}
        </modalContext.Provider>
      </motion.div>
    </dialog>
  );
};
