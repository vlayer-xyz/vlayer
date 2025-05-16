import { StepStatus } from "src/constants";
import React, { FC } from "react";
import { Flex, Text } from "@radix-ui/themes";
import { Image } from "components/atoms/Image.tsx";

type UserActionStepActionProps = {
  status: StepStatus;
  text: string;
  image?: string;
};
export const UserActionStepActions: FC<UserActionStepActionProps> = ({
  status,
  text,
  image,
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
