import React, { FC, useEffect } from "react";
import { StepStatus } from "constants/step";

type RedirectStepActionProps = {
  isVisited: boolean;
  link: string;
  status: StepStatus;
  buttonText: string;
};

export const RedirectStepActions: FC<RedirectStepActionProps> = ({
  isVisited,
  link,
  status,
}) => {
  useEffect(() => {
    if (!isVisited && status == StepStatus.Current) {
      window.location.href = link;
    }
  }, [isVisited, link, status]);

  return <></>;
};
