import { useEffect, useState } from "react";
import { useLocation, Location } from "react-router";
import { getAllSteps } from "../../app/router/steps";
import { Step } from "../../app/router/types";

export const useCurrentStep = () => {
  const location = useLocation();
  const [currentStep, setCurrentStep] = useState<Step | undefined>(undefined);

  useEffect(() => {
    setCurrentStep(getAllSteps().find(byPath(location)));
  }, [location.pathname, setCurrentStep]);
  return { currentStep };
};

const byPath = (location: Location<unknown>) => (step: Step) => {
  return step.path === location.pathname.split("/")[1];
};
