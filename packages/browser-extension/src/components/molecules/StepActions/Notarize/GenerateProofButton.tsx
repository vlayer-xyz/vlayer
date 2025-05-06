import React, { FC } from "react";
import { Text } from "@radix-ui/themes";
import { Button } from "components/atoms";
import { AnimatedContainer } from "components/molecules/AnimationContainer";

export const GenerateProofButton: FC<{
  onClick: () => void;
  isVisible: boolean;
}> = (props) => {
  return (
    <AnimatedContainer isVisible={props.isVisible}>
      <Button onClick={props.onClick} data-testid="generate-proof-button">
        <Text>Generate proof</Text>
      </Button>
    </AnimatedContainer>
  );
};
