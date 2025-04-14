import React, {
  createContext,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import { AnimatePresence, motion } from "motion/react";
import { useCurrentStep } from "../../hooks/useCurentStep";
import { STEP_KIND } from "../../utils/steps";
import { ProgressBar } from "../molecules/ProgressBar";
import { Navigation } from "./Navigation";
import { ErrorBoundary } from "react-error-boundary";
import { StepErrorBoundaryComponent } from "./ErrorBoundary";

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
  const [isSuccessStep, setIsSuccessStep] = useState(false);
  useEffect(() => {
    setIsWelcome(currentStep?.kind === STEP_KIND.WELCOME);
    setIsSuccessStep(currentStep?.kind === STEP_KIND.SUCCESS);
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
          {/* Navigation */}
          <Navigation />
          {/* Progress Bar */}
          <AnimatePresence>
            {!isWelcome && !isSuccessStep && <ProgressBar />}
          </AnimatePresence>
          <ErrorBoundary FallbackComponent={StepErrorBoundaryComponent}>
            {/* Header Icon */}
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
              {/* Title */}
              {currentStep?.title && (
                <h3 className={`header ${descClass}`}>{currentStep?.title}</h3>
              )}
              {/* Description */}
              <p className={`h-[116px] desc ${descClass}`}>{description}</p>
              {/* Content */}
              <modalContext.Provider value={{ showModal, closeModal }}>
                {children}
              </modalContext.Provider>
            </div>
          </ErrorBoundary>
        </motion.div>
      </div>
    </dialog>
  );
};
