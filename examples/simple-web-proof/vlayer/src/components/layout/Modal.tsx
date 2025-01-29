import { createContext, useCallback, useEffect, useRef, useState } from "react";
import { Link } from "react-router";
import { AnimatePresence, motion } from "motion/react";
import { useCurrentStep } from "../../hooks/useCurentStep";
import { STEP_KIND } from "../../utils/steps";
import { ProgressBar } from "../molecules/ProgressBar";

export const modalContext = createContext({
  showModal: () => {},
  closeModal: () => {},
});

export const Modal = ({ children }: { children: React.ReactNode }) => {
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
  const { currentStep } = useCurrentStep();
  const [isWelcome, setIsWelcome] = useState(false);
  useEffect(() => {
    setIsWelcome(currentStep?.kind === STEP_KIND.WELCOME);
  }, [currentStep?.kind]);

  const [descClass, setDescClass] = useState("");
  const [description, setDescription] = useState("");
  useEffect(() => {
    setDescClass("out");

    setTimeout(() => {
      setDescClass("in");
      setDescription(currentStep?.description || "");
    }, 300);
  }, [currentStep?.description]);

  return (
    <dialog className="modal" ref={modalRef}>
      <div className="modal-box bg-white rounded-2xl">
        <motion.div
          className="h-[490px] flex flex-col items-center justify-between"
          initial={{ opacity: 0, scale: 0.1 }}
          animate={{ opacity: 1, scale: 1 }}
          exit={{ opacity: 0, scale: 0.1 }}
          transition={{ ease: "easeOut", duration: 0.3 }}
        >
          <AnimatePresence>{!isWelcome && <ProgressBar />}</AnimatePresence>
          {currentStep?.backUrl && (
            <form method="dialog">
              <Link
                to={currentStep?.backUrl}
                className="absolute left-3 text-black top-3 text-xs font-normal"
              >
                Back
              </Link>
            </form>
          )}
          <AnimatePresence>
            {currentStep?.headerIcon && (
              <motion.img
                initial={{ opacity: 0, scale: 0.1 }}
                animate={{ opacity: 1, scale: 1 }}
                exit={{ opacity: 0, scale: 0.1 }}
                transition={{ ease: "easeOut", duration: 0.3 }}
                src={currentStep?.headerIcon}
                alt="Success Icon"
                className="w-[282px] h-[150px]"
              />
            )}
          </AnimatePresence>
          <div className="flex-col flex gap-4 justify-between h-[284px] mb-2">
            {currentStep?.title && (
              <h3 className={`header ${descClass}`}>{currentStep?.title}</h3>
            )}
            <p className={`h-[116px] desc ${descClass}`}>{description}</p>

            <modalContext.Provider value={{ showModal, closeModal }}>
              {children}
            </modalContext.Provider>
          </div>
        </motion.div>
      </div>
    </dialog>
  );
};
