import { StepStatus } from "constants/step";
import { match } from "ts-pattern";
import React from "react";
import { CompletedStepCircle } from "components/molecules/StepCircle/CompletedStepCircle";
import { CurrentStepCircle } from "components/molecules/StepCircle/CurrentStepCircle";
import { FurtherStepCircle } from "components/molecules/StepCircle/FurtherStepCircle";

export const StepCircle: React.FC<{
  status: StepStatus;
  index: number;
}> = ({ status, index }) => {
  return (
    <>
      {match(status)
        .with(StepStatus.Completed, () => <CompletedStepCircle />)
        .with(StepStatus.Current, () => <CurrentStepCircle index={index} />)
        .with(StepStatus.Further, () => <FurtherStepCircle index={index} />)
        .exhaustive()}
    </>
  );
};
