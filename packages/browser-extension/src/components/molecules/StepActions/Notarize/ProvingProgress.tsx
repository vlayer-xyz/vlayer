import React, { FC } from "react";
import { Flex, Progress, Text } from "@radix-ui/themes";
import { AnimatedContainer } from "components/molecules/AnimationContainer";
import { ProvingStatus } from "./types";
import { useProvingProgress } from "./ProvingProgress.hooks";

export const ProvingProgress: FC<{
  isVisible: boolean;
  provingStatus: ProvingStatus;
}> = (props) => {
  const {
    progress,
    title,
    subtitle,
    isVisible,
    stepIndex,
    stepCount,
    showDescription,
    dataTestId,
  } = useProvingProgress(props);
  return (
    <AnimatedContainer isVisible={isVisible}>
      <Flex data-testid={dataTestId} direction={"column"} gap={"3"}>
        {showDescription && (
          <>
            <Text weight={"bold"} size={"2"} color={"violet"}>
              Step {stepIndex} of {stepCount}
            </Text>

            <Text weight={"bold"} size={"3"}>
              {title}
            </Text>
            <Text weight={"light"} size={"3"}>
              {subtitle}
            </Text>
          </>
        )}
        <Progress value={progress} data-testid="proving-progress" />
      </Flex>
    </AnimatedContainer>
  );
};
