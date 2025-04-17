import { useEffect, useMemo, useState } from "react";
import { StepStatus } from "constants/step";
import { useTlsnProver } from "hooks/useTlsnProver";
import { useZkProvingState } from "hooks/useZkProvingState";
import { ProvingStatus, type NotarizeStepActionProps } from "./types";
import { useDebounceValue, useInterval } from "usehooks-ts";
import { DEFAULT_REDIRECT_DELAY_SECONDS } from "constants/defaults";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";
import { ExtensionInternalMessageType } from "src/web-proof-commons";

export const CALLOUT_DEBOUNCE_TIME = 1000;

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

  // hide the finish callout if there is an error
  useEffect(() => {
    if (error) {
      setIsFinishCalloutVisible(false);
    }
  }, [error, setIsFinishCalloutVisible]);

  useEffect(() => {
    if (isZkProvingDone) {
      setIsFinishCalloutVisible(true);
      setTimeout(() => {
        setIsFinishCalloutVisible(false);
      }, 2000);
    } else {
      setIsFinishCalloutVisible(false);
    }
    return () => {};
  }, [isZkProvingDone, setIsFinishCalloutVisible]);

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
    setIsProvingProgressVisible,
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
  const {
    isProving: isWebProving,
    error: isWebProvingError,
    proof,
  } = useTlsnProver();
  const { error: isZkProvingError } = useZkProvingState();
  const [isRedirectCalloutVisible, setIsRedirectCalloutVisible] =
    useDebounceValue(false, CALLOUT_DEBOUNCE_TIME);
  const redirectDelay =
    import.meta.env.REDIRECT_DELAY_SECONDS || DEFAULT_REDIRECT_DELAY_SECONDS;
  const [timeout, setTimeout] = useState(redirectDelay);

  // reset timeout when web proving stops
  useEffect(() => {
    if (!isWebProving) {
      setTimeout(redirectDelay);
    }
  }, [isWebProving, redirectDelay]);

  // redirection callout should be visible when web proving starts
  // aand stay till
  useEffect(() => {
    if (isWebProving) {
      setIsRedirectCalloutVisible(true);
    }
    if (!isWebProving && !proof) {
      setIsRedirectCalloutVisible(false);
    }
    if (timeout === 0) {
      setIsRedirectCalloutVisible(false);
    }
    if (isWebProvingError || isZkProvingError) {
      setIsRedirectCalloutVisible(false);
    }
  }, [
    isWebProving,
    JSON.stringify(proof),
    timeout,
    setIsRedirectCalloutVisible,
    isWebProvingError,
    isZkProvingError,
  ]);

  // start countdown when web proving starts
  useInterval(
    () => {
      setTimeout(Math.max(timeout - 1, 0));
      if (timeout === 0) {
        sendMessageToServiceWorker({
          type: ExtensionInternalMessageType.RedirectBack,
        }).catch(console.error);
      }
    },
    isWebProving ? 1000 : null,
  );

  return {
    isRedirectCalloutVisible,
    timeout,
  };
};

const useNotarizeStepActions = (props: NotarizeStepActionProps) => {
  const { onButtonClick, isButtonVisible } = useProveButton();
  const { isFinishCalloutVisible } = useFinishCallout();
  const { isProvingProgressVisible } = useProgress();
  const { provingStatus } = useProvingStatus();
  const { isRedirectCalloutVisible, timeout: redirectTimeout } =
    useRedirectCallout();
  const { error } = useTlsnProver();

  return {
    provingStatus,
    onButtonClick,
    isButtonVisible,
    isFinishCalloutVisible,
    isProvingProgressVisible,
    isRedirectCalloutVisible,
    redirectTimeout,
    errorMessage: error,
    isVisible: !props.isVisited && props.status === StepStatus.Current,
  };
};

export { useNotarizeStepActions };
