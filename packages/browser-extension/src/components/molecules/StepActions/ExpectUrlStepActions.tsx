import React, { FC } from "react";
import { StepStatus } from "constants/step";

type ExpectUrlStepActions = {
  status: StepStatus;
};
// for now there is no action associated with it. We just wait
export const ExpectUrlStepActions: FC<ExpectUrlStepActions> = () => {
  return <></>;
};
