import React from "react";

import { Flex, Text } from "@radix-ui/themes";
import { StepCircle } from "components/molecules/StepCircle/StepCircle";
import { StepStatus } from "constants/step";
import { Separator } from "components/atoms/Separator";
import { StepActions } from "components/molecules/StepActions/StepActions";
import styles from "./Step.module.css";
import { match } from "ts-pattern";
import { motion } from "framer-motion";
type StepProps = {
  label: string;
  status: StepStatus;
  index: number;
  showSeparator: boolean;
  link?: string;
  kind: "expectUrl" | "notarize" | "startPage";
};
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
    <Text
      as={motion.span}
      transition={{
        duration: 3,
      }}
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
