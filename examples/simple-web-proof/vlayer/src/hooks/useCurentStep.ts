import { useEffect, useState } from "react";
import { useLocation, Location } from "react-router";
import { steps, Step } from "../utils/steps";

export const useCurrentStep = () => {
  const location = useLocation();
  const [currentStep, setCurrentStep] = useState<Step | undefined>(undefined);

  useEffect(() => {
    setCurrentStep(steps.find(byPath(location)));
  }, [location.pathname]);
  return { currentStep };
};

const byPath = (location: Location<unknown>) => (step: Step) => {
  return step.path === location.pathname.split("/")[1];
};
