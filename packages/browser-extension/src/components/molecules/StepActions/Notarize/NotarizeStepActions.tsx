import React, { FC } from "react";
import { ProvingProgress } from "./ProvingProgress";
import { RedirectCallout } from "./RedirectCallout";
import { FinishCallout } from "./FinishCallout";
import { GenerateProofButton } from "./GenerateProofButton";
import { type NotarizeStepActionProps } from "./types";
import { Flex } from "@radix-ui/themes";
import { useNotarizeStepActions } from "./NotarizeStepActions.hooks";

export const NotarizeStepActions: FC<NotarizeStepActionProps> = (props) => {
  const {
    onButtonClick,
    isButtonVisible,
    isFinishCalloutVisible,
    isProvingProgressVisible,
    isRedirectCalloutVisible,
    isVisible,
    provingStatus,
    redirectTimeout,
  } = useNotarizeStepActions(props);
  return (
    <>
      {isVisible && (
        <Flex direction="column" gap={"4"}>
          <GenerateProofButton
            onClick={onButtonClick}
            isVisible={isButtonVisible}
          />
          <RedirectCallout
            show={isRedirectCalloutVisible}
            timeout={redirectTimeout}
          />
          <ProvingProgress
            isVisible={isProvingProgressVisible}
            provingStatus={provingStatus}
          />
          <FinishCallout isVisible={isFinishCalloutVisible} />
        </Flex>
      )}
    </>
  );
};
