// this is placeholder impolementation

import { StepStatus } from "constants/step";

export const useSteps = () => {
  return [
    {
      status: StepStatus.Completed,
      label: "Go to app.revolut.com and login",
    },
    {
      status: StepStatus.Completed,
      label: "secondStep",
    },
    {
      status: StepStatus.Current,
      label: "thirdStep",
    },
    {
      status: StepStatus.Further,
      label: "fourthStep",
    },
  ];
};
