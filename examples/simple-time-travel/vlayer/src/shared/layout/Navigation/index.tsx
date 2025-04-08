import * as React from "react";
import { ChevronLeftIcon } from "@heroicons/react/24/outline";
import { useCurrentStep } from "../../hooks/useCurrentStep";
import { useNavigate } from "react-router";

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
      className="flex gap-10 justify-between max-w-[480px] w-full"
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
          void navigate(currentStep.backUrl);
        }
      }}
      className="flex gap-1.5 justify-center items-center px-2 py-0 my-auto h-8 text-xs leading-3 text-center text-gray-800 whitespace-nowrap rounded-lg shadow-sm min-h-[32px]"
    >
      <ChevronLeftIcon className="w-4 h-4" />
      <span className="self-stretch my-auto">Back</span>
    </button>
  );
};
