import { useEffect, useState, useCallback } from "react";
import { StepStatus } from "constants/step";
import { useTlsnProver } from "hooks/useTlsnProver";
import { useZkProvingState } from "hooks/useZkProvingState";
import { ProvingStatus, type NotarizeStepActionProps } from "./types";

const useProveButton = () => {
  const { prove, isProving: isWebProving, proof } = useTlsnProver();
  const handleClick = useCallback(() => {
    prove().catch((error) => {
      console.error("Error during prove:", error);
    });
  }, []);
  return {
    onButtonClick: handleClick,
    isButtonVisible: !isWebProving && !proof,
  };
};

const useFinishCallout = () => {
  const [isFinishCalloutVisible, setIsFinishCalloutVisible] = useState(false);
  const { isDone: isZkProvingDone } = useZkProvingState();

  useEffect(() => {
    if (isZkProvingDone) {
      setIsFinishCalloutVisible(true);
      setTimeout(() => {
        setIsFinishCalloutVisible(false);
      }, 2000);
    }
    return () => {};
  }, [isZkProvingDone]);

  return {
    isFinishCalloutVisible,
  };
};

const useProgress = () => {
  const { isProving: isZkProving, isDone: isZkProvingDone } =
    useZkProvingState();
  const { isProving: isWebProving } = useTlsnProver();
  const [isProvingProgressVisible, setIsProvingProgressVisible] =
    useState(false);

  useEffect(() => {
    if (isZkProvingDone) {
      setTimeout(() => {
        setIsProvingProgressVisible(false);
      }, 2000);
    }
    if (isWebProving || isZkProving) {
      setIsProvingProgressVisible(true);
    }
    return () => {};
  }, [isZkProvingDone, isWebProving, isZkProving]);

  return {
    isProvingProgressVisible,
  };
};

const useProvingStatus = () => {
  const { isProving: isWebProving } = useTlsnProver();
  const { isProving: isZkProving, isDone: isZkProvingDone } =
    useZkProvingState();

  const [provingStatus, setProvingStatus] = useState(ProvingStatus.NotStared);

  useEffect(() => {
    if (isZkProvingDone) {
      setProvingStatus(ProvingStatus.Done);
    } else if (isZkProving) {
      setProvingStatus(ProvingStatus.Zk);
    } else if (isWebProving) {
      setProvingStatus(ProvingStatus.Web);
    } else {
      setProvingStatus(ProvingStatus.NotStared);
    }
    return () => {};
  }, [isZkProving, isZkProvingDone, isWebProving]);

  return {
    provingStatus,
  };
};

const useNotarizeStepActions = (props: NotarizeStepActionProps) => {
  const { onButtonClick, isButtonVisible } = useProveButton();
  const { isFinishCalloutVisible } = useFinishCallout();
  const { isProvingProgressVisible } = useProgress();
  const { isProving: isWebProving } = useTlsnProver();
  const { provingStatus } = useProvingStatus();

  return {
    provingStatus,
    onButtonClick,
    isButtonVisible,
    isFinishCalloutVisible,
    isProvingProgressVisible,
    isRedirectCalloutVisible: isWebProving,
    isVisible: !props.isVisited && props.status === StepStatus.Current,
  };
};

export { useNotarizeStepActions };
