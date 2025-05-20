import { StepStatus } from "src/constants";
import React, { FC } from "react";
import { Flex, Text } from "@radix-ui/themes";
import { Image } from "components/atoms/Image.tsx";
import { WebProofStepUserAction } from "src/web-proof-commons/types/message";

type UserActionStepActionProps = {
  status: StepStatus;
  instruction: WebProofStepUserAction["instruction"];
};
export const UserActionStepActions: FC<UserActionStepActionProps> = ({
  status,
  instruction: { text, image },
}) => {
  const isVisible = status === StepStatus.Current;
  return (
    <>
      {isVisible && (
        <Flex direction="column" gap={"4"}>
          {image && <Image src={image} alt="User action instruction" />}
          <Text>{text}</Text>
        </Flex>
      )}
    </>
  );
};
