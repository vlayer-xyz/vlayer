import React, {
  createContext,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import { AnimatePresence, motion } from "motion/react";
import { useCurrentStep } from "../../hooks/useCurentStep";
import { STEP_KIND } from "../../../app/router/steps";
import { ProgressBar } from "../ProgressBar";
import { Navigation } from "../Navigation";
import { motionConfig } from "./Modal.animations";
import styles from "./Modal.module.css";

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
        <motion.div className={styles.innerModal} {...motionConfig}>
          <Navigation />
          <AnimatePresence>
            {!isWelcome && !isSuccessStep && <ProgressBar />}
          </AnimatePresence>
          <AnimatePresence>
            {currentStep?.headerIcon && (
              <motion.img
                src={currentStep?.headerIcon}
                alt="Success Icon"
                className={styles.headerIcon}
                {...motionConfig}
              />
            )}
          </AnimatePresence>
          <div className={styles.content}>
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
