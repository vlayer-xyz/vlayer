import React, { FC, useEffect, useState } from "react";
import { Callout, Flex, Text, Progress } from "@radix-ui/themes";
import { InfoCircledIcon } from "@radix-ui/react-icons";
import { StepStatus } from "constants/step";
import { Button } from "components/atoms";
import { useTlsnProver } from "hooks/useTlsnProver";
import browser from "webextension-polyfill";

type NotarizeStepActionProps = {
  isVisited: boolean;
  buttonText: string;
  link: string;
  status: StepStatus;
};

const RedirectCallout: FC = () => {
  const [timeout, setTimeout] = useState(4);
  const [show, setShow] = useState(true);
  useEffect(() => {
    const interval = setInterval(() => {
      setTimeout((timeout) => {
        if (timeout === 0) {
          // hide callout
          setShow(false);
          // tell service worker to redirect back to orginal page
          browser.runtime.sendMessage({
            type: "redirectBack",
          });
          clearInterval(interval);
        }
        return timeout - 1;
      });
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  return show ? (
    <Callout.Root>
      <Callout.Icon>
        <InfoCircledIcon />
      </Callout.Icon>
      <Callout.Text>
        You will be redirected to www.user.dapp.com in a {timeout} second
        {timeout > 1 ? "s" : ""}.
      </Callout.Text>
    </Callout.Root>
  ) : (
    <></>
  );
};

const ProvingProgress = () => {
  const { proof } = useTlsnProver();
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    let interval: Timer;
    if (!proof) {
      interval = setInterval(() => {
        setProgress((progress) => {
          const newProgress = progress + 1;
          if (proof) {
            clearInterval(interval!);
            return progress;
          }
          return newProgress;
        });
      }, 1200);
    } else {
      setProgress(100);
    }

    return () => {
      if (interval) {
        clearInterval(interval);
      }
    };
  }, [proof]);
  return (
    <>
      <Text weight={"bold"} size={"3"}>
        Generating Web Proof for user.dapp
      </Text>
      <Text weight={"light"} size={"3"}>
        This usually takes 1-2 min. Don’t close your browser.
      </Text>
      <Progress value={progress} />
    </>
  );
};

const FinishCallout: FC = () => {
  return (
    <Callout.Root>
      <Callout.Icon>
        <InfoCircledIcon />
      </Callout.Icon>
      <Callout.Text>Generating proof has been finished</Callout.Text>
    </Callout.Root>
  );
};

export const NotarizeStepActions: FC<NotarizeStepActionProps> = ({
  isVisited,
  status,
}) => {
  const { prove, isProving, proof } = useTlsnProver();
  const [showProgress, setShowProgress] = useState(false);
  // defer progress hiding
  useEffect(() => {
    if (proof) {
      setTimeout(() => {
        setShowProgress(false);
      }, 2000);
    }
    if (isProving) {
      setShowProgress(true);
    }
  }, [proof, isProving]);
  return isVisited || status === StepStatus.Further ? (
    <></>
  ) : (
    <Flex direction="column" gap={"4"}>
      {!proof && !isProving && (
        <Button onClick={prove}>
          <Text>Generate proof </Text>
        </Button>
      )}
      {isProving && (
        <>
          <RedirectCallout />
        </>
      )}
      {showProgress && <ProvingProgress />}
      {proof ? <FinishCallout /> : ""}
    </Flex>
  );
};
