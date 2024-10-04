import React, { FC } from "react";
import { Button, Link } from "@radix-ui/themes";
import { StepStatus } from "constants/step";

type NotarizeStepActionProps = {
  isVisited: boolean;
  buttonText: string;
  link: string;
  status: StepStatus;
};

export const NotarizeStepActions: FC<NotarizeStepActionProps> = ({
  isVisited,
  buttonText,
  link,
  status,
}) => {
  return isVisited || status !== StepStatus.Current ? (
    <></>
  ) : (
    <Button variant={"soft"}>
      {" "}
      <Link href={link}> {buttonText} </Link>
    </Button>
  );
};
