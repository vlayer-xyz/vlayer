import React, { FC } from "react";
import { Button, Link, Grid } from "@radix-ui/themes";
import { StepStatus } from "constants/step";

type StartPageStepActionProps = {
  isVisited: boolean;
  link: string;
  status: StepStatus;
  buttonText: string;
};

export const StartPageStepActions: FC<StartPageStepActionProps> = ({
  isVisited,
  buttonText,
  link,
  status,
}) => {
  return isVisited || status !== StepStatus.Current ? (
    <></>
  ) : (
    <Grid columns={"5"}>
      <Button
        variant={"soft"}
        style={{
          gridColumn: "1 / 5",
          marginBottom: "1rem",
        }}
      >
        {" "}
        <Link href={link}> {buttonText} </Link>
      </Button>
    </Grid>
  );
};
