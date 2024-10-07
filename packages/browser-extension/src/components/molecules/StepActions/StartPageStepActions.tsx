import React, { FC } from "react";
import { Text, Grid } from "@radix-ui/themes";
import { StepStatus } from "constants/step";
import { Button } from "components/atoms";
import browser from "webextension-polyfill";

type StartPageStepActionProps = {
  isVisited: boolean;
  link: string;
  status: StepStatus;
  buttonText: string;
};

export const StartPageStepActions: FC<StartPageStepActionProps> = ({
  isVisited,
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
        // open app we gona take proof from in new tab
        onClick={async () => {
          await browser.tabs.create({
            url: link,
          });
        }}
      >
        <Text>Redirect</Text>
      </Button>
    </Grid>
  );
};
