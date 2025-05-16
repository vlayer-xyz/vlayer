import { StepStatus } from "constants/step";
import React, { FC } from "react";

type UserActionStepActionProps = {
  isVisited: boolean;
  link: string;
  status: StepStatus;
  buttonText: string;
};

export const UserActionStepActions: FC<UserActionStepActionProps> = ({ }) => {
  return <></>;
};
