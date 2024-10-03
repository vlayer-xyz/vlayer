import React from "react";

type StepProps = {
  label: string;
  status: StepStatus;
  index: number;
  showSeparator: boolean;
};

import { Flex, Text, Grid, Box } from "@radix-ui/themes";
import { StepCircle } from "components/molecules/StepCircle";
import { StepStatus } from "constants/step";
import { Separator } from "components/atoms/Separator";
export const Step = (props: StepProps) => {
  return (
    <Flex gap={"4"} justify={"start"}>
      <Flex direction={"column"}>
        <StepCircle {...props} />
        {props.showSeparator && <Separator />}
      </Flex>
      <Text size={"4"} weight={"medium"}>
        {props.label}
      </Text>
    </Flex>
  );
};
