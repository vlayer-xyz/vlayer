import { StepStatus } from "src/constants";
import React, { FC } from "react";
import { Flex, Text } from "@radix-ui/themes";
import { UserActionButton } from "components/molecules/StepActions/UserAction/UserActionButton.tsx";

type UserActionStepActionProps = {
  isVisited: boolean;
  link: string;
  status: StepStatus;
  text: string;
  image?: string;
};
export const UserActionStepActions: FC<UserActionStepActionProps> = ({
  isVisited,
  status,
  text,
  image,
}) => {
  const isVisible = !isVisited && status === StepStatus.Current;
  return (
    <>
      {isVisible && (
        <Flex direction="column" gap={"4"}>
          {image && <img src={image} alt="User action instruction" />}
          <Text>{text}</Text>
          <UserActionButton isVisible={isVisible} onClick={() => {}}>
            Proceed
          </UserActionButton>
        </Flex>
      )}
    </>
  );
};
