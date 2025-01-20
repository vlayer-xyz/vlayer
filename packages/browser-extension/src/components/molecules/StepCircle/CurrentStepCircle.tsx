import { Circle } from "components/atoms/Circle";
import React from "react";
import { Text } from "@radix-ui/themes";

export const CurrentStepCircle = ({ index }: { index: number }) => {
  return (
    <Circle isSolid={false} isDisabled={false}>
      <Text size={"2"} weight={"bold"}>
        {index}
      </Text>
    </Circle>
  );
};
