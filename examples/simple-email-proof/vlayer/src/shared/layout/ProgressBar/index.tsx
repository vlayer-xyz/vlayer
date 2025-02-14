import { useCurrentStep } from "../../hooks/useCurentStep";
import { motion } from "motion/react";
import styles from "./ProgressBar.module.css";
import motionConfig from "./ProgressBar.animations";

export const ProgressBar = () => {
  const { currentStep } = useCurrentStep();

  const activeStepClass = (index: number) =>
    currentStep?.index !== undefined && currentStep?.index >= index
      ? "step-primary"
      : "";

  return (
    <motion.ul className={styles.progressBar} {...motionConfig}>
      <li className={activeStepClass(1)}>Connect Wallet</li>
      <li className={activeStepClass(2)}>Get data from X</li>
      <li className={activeStepClass(3)}>Mint NFT</li>
    </motion.ul>
  );
};
