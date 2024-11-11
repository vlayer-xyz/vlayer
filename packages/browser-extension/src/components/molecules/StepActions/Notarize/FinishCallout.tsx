import { Callout } from "@radix-ui/themes";
import { InfoCircledIcon } from "@radix-ui/react-icons";
import React, { FC } from "react";
import { AnimatedContainer } from "components/molecules/AnimationContainer";

export const FinishCallout: FC<{ isVisible: boolean }> = (props) => {
  return (
    <AnimatedContainer isVisible={props.isVisible} data-testid="finish-callout">
      <Callout.Root>
        <Callout.Icon>
          <InfoCircledIcon />
        </Callout.Icon>
        <Callout.Text>Generating proof has been finished</Callout.Text>
      </Callout.Root>
    </AnimatedContainer>
  );
};
