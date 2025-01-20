import { Circle } from "components/atoms/Circle";
import React from "react";
import { Text } from "@radix-ui/themes";

export const FurtherStepCircle = ({ index }: { index: number }) => {
  return (
    <Circle isSolid={false} isDisabled={true}>
      <Text size={"2"} weight={"bold"}>
        {index}
      </Text>
    </Circle>
  );
};
