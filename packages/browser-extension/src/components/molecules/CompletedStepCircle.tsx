import React from "react";
import { Circle } from "components/atoms/Circle";
import { CheckIcon } from "@radix-ui/react-icons";

export const CompletedStepCircle = () => {
  return (
    <Circle isSolid={true} isDisabled={false}>
      <CheckIcon width={20} height={20} color="white" />
    </Circle>
  );
};
