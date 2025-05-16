import React from "react";

import { Flex, Text } from "@radix-ui/themes";
import { StepCircle } from "components/molecules/StepCircle/StepCircle";
import { StepStatus } from "constants/step";
import { Separator } from "components/atoms/Separator";
import { StepActions } from "components/molecules/StepActions/StepActions";
import styles from "./Step.module.css";
import { match } from "ts-pattern";
import { ExtensionStep, WebProofStep } from "src/web-proof-commons";

export type StepProps = {
  step: WebProofStep;
  label: string;
  status: StepStatus;
  index: number;
  showSeparator: boolean;
  link?: string;
  kind: ExtensionStep;
};

const StepStatusIndicator = (props: StepProps) => {
  return (
    <Flex
      direction="column"
      align="center"
      style={{
        height: "100%",
      }}
      data-testid={`step-${props.kind}`}
      data-status={props.status}
    >
      <StepCircle {...props} />
      {props.showSeparator && <Separator />}
    </Flex>
  );
};

const StepLabel = (props: StepProps) => {
  return (
    <Text
      size={"3"}
      weight={"medium"}
      className={match(props.status)
        .with(StepStatus.Further, () => styles.textFurther)
        .with(StepStatus.Completed, () => styles.textCompleted)
        .with(StepStatus.Current, () => styles.textCurrent)
        .exhaustive()}
    >
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
