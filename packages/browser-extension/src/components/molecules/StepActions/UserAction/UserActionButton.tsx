import React, { FC } from "react";
import { Text } from "@radix-ui/themes";
import { Button } from "components/atoms";
import { AnimatedContainer } from "components/molecules/AnimationContainer";

export const UserActionButton: FC<{
  onClick: () => void;
  isVisible: boolean;
  children: React.ReactNode;
}> = ({ onClick, isVisible, children }) => {
  return (
    <AnimatedContainer isVisible={isVisible}>
      <Button onClick={onClick} data-testid="user-action-button">
        <Text>{children}</Text>
      </Button>
    </AnimatedContainer>
  );
};
