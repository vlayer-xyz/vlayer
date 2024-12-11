import { useSteps } from "hooks/useSteps";
import { Step } from "components/molecules/Step";
import React from "react";
export const Steps = () => {
  const steps = useSteps();
  return (
    <div data-testid="steps">
      {steps.map((step, index) => [
        <Step
          {...step}
          index={index}
          key={`${step.label}`}
          showSeparator={index < steps.length - 1}
          link={step.link}
        />,
      ])}
    </div>
  );
};
