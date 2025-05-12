import React, { FC } from "react";
import { Callout } from "@radix-ui/themes";
import { InfoCircledIcon } from "@radix-ui/react-icons";

import { AnimatedContainer } from "components/molecules/AnimationContainer";

export const RedirectCallout: FC<{ show: boolean; timeout: number }> = ({
  show,
  timeout,
}) => {
  return (
    <AnimatedContainer isVisible={show} data-testid="redirect-callout">
      <Callout.Root>
        <Callout.Icon>
          <InfoCircledIcon />
        </Callout.Icon>
        <Callout.Text>
          You will be redirected back in <b data-testid="timeout">{timeout}</b>{" "}
          second
          {timeout !== 1 ? "s" : ""}.
        </Callout.Text>
      </Callout.Root>
    </AnimatedContainer>
  );
};
