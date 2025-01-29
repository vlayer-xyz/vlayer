import { useEffect, useState } from "react";
import { useLocation, Location } from "react-router";
import { steps, Step } from "../utils/steps";

export const useCurrentStep = () => {
  const location = useLocation();
  const [currentStep, setCurrentStep] = useState<
    (Step & { index: number }) | undefined
  >(undefined);

  useEffect(() => {
    setCurrentStep(steps.map(setIndex).find(byPath(location)));
  }, [location.pathname]);
  return { currentStep };
};

const setIndex = (step: Step, index: number) => {
  return { ...step, index };
};

const byPath = (location: Location<unknown>) => (step: Step) => {
  return step.path === location.pathname.split("/")[1];
};
