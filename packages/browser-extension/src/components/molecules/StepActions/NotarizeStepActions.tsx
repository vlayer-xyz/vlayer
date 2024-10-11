import React, { FC, useEffect, useState } from "react";
import { Callout, Flex, Progress, Text } from "@radix-ui/themes";
import { InfoCircledIcon } from "@radix-ui/react-icons";
import { StepStatus } from "constants/step";
import { Button } from "components/atoms";
import { useTlsnProver } from "hooks/useTlsnProver";
import { AnimatePresence, motion } from "framer-motion";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";
import { ExtensionMessageType } from "@vlayer/web-proof-commons";

type NotarizeStepActionProps = {
  isVisited: boolean;
  buttonText: string;
  link: string;
  status: StepStatus;
};

const RedirectCallout: FC = () => {
  const [timeout, setTimeout] = useState(10);
  const [show, setShow] = useState(true);
  useEffect(() => {
    const interval = setInterval(() => {
      setTimeout((timeout) => {
        if (timeout === 0) {
          // hide callout
          setShow(false);
          // tell service worker to redirect back to orginal page
          sendMessageToServiceWorker({
            type: ExtensionMessageType.RedirectBack,
          });
          clearInterval(interval);
        }
        return timeout - 1;
      });
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <AnimatePresence>
      {show && (
        <motion.div
          initial={{ opacity: 0, height: 0 }}
          animate={{ opacity: 1, height: "auto" }}
          exit={{ opacity: 0, height: 0 }}
        >
          <Callout.Root>
            <Callout.Icon>
              <InfoCircledIcon />
            </Callout.Icon>
            <Callout.Text>
              You will be redirected back in <b>{timeout}</b> second
              {timeout != 1 ? "s" : ""}.
            </Callout.Text>
          </Callout.Root>
        </motion.div>
      )}
    </AnimatePresence>
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
          const newProgress = Math.min(100, progress + 1);
          if (proof) {
            clearInterval(interval!);
            return progress;
          }
          return newProgress;
        });
      }, 600);
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
    <Flex direction={"column"} gap={"3"}>
      <Text weight={"bold"} size={"3"}>
        Generating Web Proof
      </Text>
      <Text weight={"light"} size={"3"}>
        This usually takes up to 1 min. Don’t close your browser.
      </Text>
      <Progress value={progress} />
    </Flex>
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
        <Button onClick={prove} data-testid="prove-button">
          <Text>Generate proof </Text>
        </Button>
      )}
      {isProving && (
        <>
          <RedirectCallout />
        </>
      )}
      <AnimatePresence>
        {showProgress && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: "auto" }}
            exit={{ opacity: 0, height: 0 }}
          >
            <ProvingProgress />
          </motion.div>
        )}
      </AnimatePresence>
      <AnimatePresence>
        {proof && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: "auto" }}
            exit={{ opacity: 0, height: 0 }}
          >
            <FinishCallout />
          </motion.div>
        )}
      </AnimatePresence>
    </Flex>
  );
};
