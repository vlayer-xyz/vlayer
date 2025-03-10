import { useCurrentStep } from "../../hooks/useCurentStep";
import { motion } from "motion/react";
import motionConfig from "./ProgressBar.animations";

export const ProgressBar = () => {
  const { currentStep } = useCurrentStep();

  const activeStepClass = (index: number) =>
    currentStep?.index !== undefined && currentStep?.index >= index
      ? "step-primary"
      : "";

  return (
    <motion.ul className="steps w-full" {...motionConfig}>
      <li className={`step text-black text-xs ${activeStepClass(1)}`}>
        Connect Wallet
      </li>
      <li className={`step text-black text-xs ${activeStepClass(2)}`}>
        Send email
      </li>
      <li className={`step text-black text-xs ${activeStepClass(3)}`}>
        Mint NFT
      </li>
    </motion.ul>
  );
};
