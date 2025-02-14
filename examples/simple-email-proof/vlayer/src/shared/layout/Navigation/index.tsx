import * as React from "react";
import { ChevronLeftIcon } from "@heroicons/react/24/outline";
import { useCurrentStep } from "../../hooks/useCurentStep";
import { useNavigate } from "react-router";
import styles from "./Navigation.module.css";

export const Navigation: React.FC = () => {
  return (
    <Navbar>
      <BackButton />
    </Navbar>
  );
};

export const Navbar: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const { currentStep } = useCurrentStep();

  return (
    <nav
      className={styles.navbar}
      style={{ opacity: currentStep?.backUrl ? 1 : 0 }}
    >
      {children}
    </nav>
  );
};

export const BackButton: React.FC = () => {
  const { currentStep } = useCurrentStep();
  const navigate = useNavigate();

  return (
    <button
      onClick={() => {
        if (currentStep?.backUrl) {
          navigate(currentStep.backUrl);
        }
      }}
      className={styles.backButton}
    >
      <ChevronLeftIcon />
      <span>Back</span>
    </button>
  );
};
