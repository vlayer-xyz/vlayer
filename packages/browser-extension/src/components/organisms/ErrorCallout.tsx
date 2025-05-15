import React, { FC, useEffect, useState } from "react";
import { Callout } from "@radix-ui/themes";
import { ExclamationTriangleIcon } from "@radix-ui/react-icons";
import { AnimatedContainer } from "components/molecules/AnimationContainer";
import { useTlsnProver } from "hooks/useTlsnProver";

export const useErrorCallout = () => {
  const [isErrorCalloutVisible, setIsErrorCalloutVisible] = useState(false);
  const { error } = useTlsnProver();
  useEffect(() => {
    setIsErrorCalloutVisible(!!error);
  }, [error]);
  return {
    isErrorCalloutVisible,
    errorMessage: error,
  };
};

export const ErrorCalloutContainer: FC = () => {
  const { isErrorCalloutVisible, errorMessage } = useErrorCallout();
  return (
    <ErrorCalloutPresentational
      isVisible={isErrorCalloutVisible}
      errorMessage={errorMessage ?? ""}
    />
  );
};

export const ErrorCalloutPresentational: FC<{
  isVisible: boolean;
  errorMessage: string;
}> = (props) => {
  return (
    <AnimatedContainer isVisible={props.isVisible}>
      <Callout.Root
        color="red"
        variant="surface"
        style={{
          marginTop: "7rem",
          textAlign: "center",
          padding: "1rem",
          marginLeft: "2rem",
          marginRight: "2rem",
        }}
      >
        <Callout.Icon>
          <ExclamationTriangleIcon />
        </Callout.Icon>
        <Callout.Text data-testid="error-message">
          {props.errorMessage}
        </Callout.Text>
      </Callout.Root>
    </AnimatedContainer>
  );
};

export const ErrorCallout = () => {
  return <ErrorCalloutContainer />;
};
