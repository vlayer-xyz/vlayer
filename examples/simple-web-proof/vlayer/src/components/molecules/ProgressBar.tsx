import { useCurrentStep } from "../../hooks/useCurentStep";

export const ProgressBar = () => {
  const { currentStep } = useCurrentStep();
  return (
    <ul className="steps w-full">
      <li
        className={`step ${currentStep?.index !== undefined && currentStep?.index >= 1 ? "step-primary" : ""} text-black text-xs`}
      >
        Connect Wallet
      </li>
      <li
        className={`step ${currentStep?.index !== undefined && currentStep?.index >= 2 ? "step-primary" : ""} text-black text-xs`}
      >
        Get data from X
      </li>
      <li
        className={`step ${currentStep?.index !== undefined && currentStep?.index >= 3 ? "step-primary" : ""} text-black text-xs`}
      >
        Mint NFT
      </li>
    </ul>
  );
};
