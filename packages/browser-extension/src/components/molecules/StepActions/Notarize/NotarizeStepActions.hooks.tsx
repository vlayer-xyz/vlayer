import { useEffect, useMemo } from "react";
import { StepStatus } from "constants/step";
import { useTlsnProver } from "hooks/useTlsnProver";
import { useZkProvingState } from "hooks/useZkProvingState";
import { ProvingStatus, type NotarizeStepActionProps } from "./types";
import { useDebounceValue } from "usehooks-ts";

export const CALLOUT_DEBOUNCE_TIME = 1500;

const useProveButton = () => {
  const { prove, isProving: isWebProving, proof } = useTlsnProver();
  return {
    onButtonClick: () => {
      prove().catch((error) => {
        console.error("error generating tlsn proof", error);
      });
    },
    isButtonVisible: !isWebProving && !proof,
  };
};

const useFinishCallout = () => {
  const [isFinishCalloutVisible, setIsFinishCalloutVisible] = useDebounceValue(
    false,
    CALLOUT_DEBOUNCE_TIME,
  );
  const { isDone: isZkProvingDone } = useZkProvingState();
  const { error } = useTlsnProver();
  useEffect(() => {
    if (error) {
      setIsFinishCalloutVisible(false);
    }
  }, [error]);
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
  const {
    isProving: isZkProving,
    isDone: isZkProvingDone,
    error: isZkProvingError,
  } = useZkProvingState();
  const { isProving: isWebProving, error: isWebProvingError } = useTlsnProver();
  const [isProvingProgressVisible, setIsProvingProgressVisible] =
    useDebounceValue(false, CALLOUT_DEBOUNCE_TIME);

  useEffect(() => {
    const isError = isWebProvingError || isZkProvingError;

    if (isWebProving || isZkProving) {
      setIsProvingProgressVisible(true);
    }
    if (isError) {
      setIsProvingProgressVisible(false);
    }
    if (isZkProvingDone) {
      setTimeout(() => {
        setIsProvingProgressVisible(false);
      }, 2000);
    }
    if (!isZkProving && !isWebProving) {
      setIsProvingProgressVisible(false);
    }
    return () => {};
  }, [
    isZkProvingDone,
    isWebProving,
    isZkProving,
    isWebProvingError,
    isZkProvingError,
  ]);

  return {
    isProvingProgressVisible,
  };
};

const useProvingStatus = () => {
  const { isProving: isWebProving } = useTlsnProver();
  const { isProving: isZkProving, isDone: isZkProvingDone } =
    useZkProvingState();

  const provingStatus = useMemo(() => {
    if (isZkProvingDone) {
      return ProvingStatus.Done;
    }
    if (isZkProving) {
      return ProvingStatus.Zk;
    }
    if (isWebProving) {
      return ProvingStatus.Web;
    }
    return ProvingStatus.NotStared;
  }, [isZkProving, isZkProvingDone, isWebProving]);

  return {
    provingStatus,
  };
};

const useRedirectCallout = () => {
  const { isProving: isWebProving, error: isWebProvingError } = useTlsnProver();
  const { error: isZkProvingError } = useZkProvingState();
  const [isRedirectCalloutVisible, setIsRedirectCalloutVisible] =
    useDebounceValue(false, CALLOUT_DEBOUNCE_TIME);

  useEffect(() => {
    const isError = isWebProvingError || isZkProvingError;
    if (isWebProving && !isError) {
      setIsRedirectCalloutVisible(true);
    } else if (isError) {
      setIsRedirectCalloutVisible(false);
    }
  }, [isWebProving, isWebProvingError, isZkProvingError]);

  return {
    isRedirectCalloutVisible,
  };
};

const useNotarizeStepActions = (props: NotarizeStepActionProps) => {
  const { onButtonClick, isButtonVisible } = useProveButton();
  const { isFinishCalloutVisible } = useFinishCallout();
  const { isProvingProgressVisible } = useProgress();
  const { provingStatus } = useProvingStatus();
  const { isRedirectCalloutVisible } = useRedirectCallout();
  const { error } = useTlsnProver();

  return {
    provingStatus,
    onButtonClick,
    isButtonVisible,
    isFinishCalloutVisible,
    isProvingProgressVisible,
    isRedirectCalloutVisible,
    errorMessage: error,
    isVisible: !props.isVisited && props.status === StepStatus.Current,
  };
};

export { useNotarizeStepActions };
