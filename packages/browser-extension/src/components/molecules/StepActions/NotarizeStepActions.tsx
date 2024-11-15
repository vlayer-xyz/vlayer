import React, { FC, useEffect, useState } from "react";
import { Callout, Flex, Progress, Text } from "@radix-ui/themes";
import { InfoCircledIcon } from "@radix-ui/react-icons";
import { StepStatus } from "constants/step";
import { Button } from "components/atoms";
import { useTlsnProver } from "hooks/useTlsnProver";
import { AnimatePresence, motion } from "framer-motion";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";
import {
  ExtensionMessageType,
  ZkProvingStatus,
} from "../../../web-proof-commons";

import { DEFAULT_REDIRECT_DELAY_SECONDS } from "constants/defaults";
import { useInterval } from "usehooks-ts";
import { useZkProvingState } from "hooks/useZkProvingState";

type NotarizeStepActionProps = {
  isVisited: boolean;
  buttonText: string;
  link: string;
  status: StepStatus;
};

const RedirectCallout: FC = () => {
  const [timeout, setTimeout] = useState(
    import.meta.env.REDIRECT_DELAY_SECONDS || DEFAULT_REDIRECT_DELAY_SECONDS,
  );
  const [show, setShow] = useState(true);

  useInterval(
    () => {
      setTimeout(timeout - 1);
      if (timeout === 0) {
        // hide callout
        setShow(false);
        // tell service worker to redirect back to original page
        sendMessageToServiceWorker({
          type: ExtensionMessageType.RedirectBack,
        }).catch(console.error);
      }
    },
    show ? 1000 : null,
  );

  return (
    <AnimatePresence>
      {show && (
        <motion.div
          initial={{ opacity: 0, height: 0 }}
          animate={{ opacity: 1, height: "auto" }}
          exit={{ opacity: 0, height: 0 }}
        >
          <Callout.Root>
            3
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

const ProvingStepWeb = ({ progress }: { progress: number }) => {
  return (
    <>
      <Text
        weight={"bold"}
        size={"2"}
        color={"violet"}
        data-testid="step_proving_web"
      >
        Step 1 of 2
      </Text>
      <Text weight={"bold"} size={"3"}>
        Generating Web Proof
      </Text>
      <Text weight={"light"} size={"3"}>
        This usually takes up to 1 min. Don’t close your browser.
      </Text>
      <Progress value={progress} data-testid={"proving-progress"} />
    </>
  );
};

const ProvingStepZk = ({ progress }: { progress: number }) => {
  return (
    <>
      <Text
        weight={"bold"}
        size={"2"}
        color={"violet"}
        data-testid="step_proving_zk"
      >
        Step 2 of 2
      </Text>
      <Text weight={"bold"} size={"3"}>
        Generating ZK Proof
      </Text>
      <Text weight={"light"} size={"3"}>
        This usually takes up to 1 min. Don’t close your browser.
      </Text>
      <Progress value={progress} data-testid={"proving-progress"} />
    </>
  );
};

const ProvingProgress = () => {
  const { isProving: isWebProving } = useTlsnProver();
  const [progress, setProgress] = useState(0);
  const {
    value: zkProvingStatus,
  }: {
    value: ZkProvingStatus;
  } = useZkProvingState();
  useInterval(
    () => {
      if (zkProvingStatus === ZkProvingStatus.Done) {
        setProgress(100);
      } else {
        setProgress(progress + 1);
      }
    },
    progress == 100 ? null : 600,
  );
  return (
    <Flex direction={"column"} gap={"3"}>
      {isWebProving ? (
        <ProvingStepWeb progress={progress} />
      ) : (
        <ProvingStepZk progress={progress} />
      )}
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
  const { prove, isProving: isWebProving, proof } = useTlsnProver();
  const { isProving: isZkProving, value } = useZkProvingState();

  const [showProgress, setShowProgress] = useState(false);
  const handleClick = () => {
    prove().catch((error) => {
      console.error("Error during prove:", error);
    });
  };
  // defer progress hiding
  useEffect(() => {
    if (value === ZkProvingStatus.Done) {
      setTimeout(() => {
        setShowProgress(false);
      }, 2000);
    }
    if (isWebProving || isZkProving) {
      setShowProgress(true);
    }
  }, [value, isWebProving, isZkProving]);
  return isVisited || status === StepStatus.Further ? (
    <></>
  ) : (
    <Flex direction="column" gap={"4"}>
      {!proof && !isWebProving && (
        <Button onClick={handleClick} data-testid="prove-button">
          <Text>Generate proof </Text>
        </Button>
      )}
      {isWebProving && (
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
