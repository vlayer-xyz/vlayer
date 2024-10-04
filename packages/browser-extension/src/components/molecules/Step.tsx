import React from "react";

type StepProps = {
  label: string;
  status: StepStatus;
  index: number;
  showSeparator: boolean;
  link?: string;
  kind: "expectUrl" | "notarize" | "startPage";
};

import { Flex, Text } from "@radix-ui/themes";
import { StepCircle } from "components/molecules/StepCircle/StepCircle";
import { StepStatus } from "constants/step";
import { Separator } from "components/atoms/Separator";
import { StepActions } from "components/molecules/StepActions/StepActions";

const StepStatusIndicator = (props: StepProps) => {
  return (
    <Flex
      direction="column"
      align="center"
      style={{
        height: "100%",
      }}
    >
      <StepCircle {...props} />

      {props.showSeparator && <Separator />}
    </Flex>
  );
};

const StepLabel = (props: StepProps) => {
  return (
    <Text size={"3"} weight={"medium"}>
      {props.label}
    </Text>
  );
};

export const Step = (props: StepProps) => {
  return (
    <Flex gap={"4"} justify={"start"}>
      <Flex direction={"column"}>
        <StepStatusIndicator {...props} />
      </Flex>
      <Flex direction={"column"} gap={"4"}>
        <StepLabel {...props} />
        <StepActions {...props} />
      </Flex>
    </Flex>
  );
};
