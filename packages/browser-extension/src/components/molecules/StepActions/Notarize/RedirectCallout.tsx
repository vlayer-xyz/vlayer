import React, { FC, useState, useEffect } from "react";
import { Callout } from "@radix-ui/themes";
import { InfoCircledIcon } from "@radix-ui/react-icons";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";
import { ExtensionMessageType } from "../../../../web-proof-commons";
import { DEFAULT_REDIRECT_DELAY_SECONDS } from "constants/defaults";
import { useInterval } from "usehooks-ts";
import { AnimatedContainer } from "components/molecules/AnimationContainer";

const useRedirectCallout = ({ isVisible }: { isVisible: boolean }) => {
  const [timeout, setTimeout] = useState(2);
  // import.meta.env.REDIRECT_DELAY_SECONDS || DEFAULT_REDIRECT_DELAY_SECONDS, 5

  const [show, setShow] = useState(isVisible);

  useEffect(() => {
    setShow(isVisible);
  }, [isVisible]);

  useInterval(
    () => {
      setTimeout(timeout - 1);
      if (timeout === 0) {
        setShow(false);
        sendMessageToServiceWorker({
          type: ExtensionMessageType.RedirectBack,
        }).catch(console.error);
      }
    },
    show ? 1000 : null,
  );

  return {
    show,
    timeout,
  };
};
export const RedirectCallout: FC<{ isVisible: boolean }> = (props) => {
  const { show, timeout } = useRedirectCallout(props);
  return (
    <AnimatedContainer isVisible={show}>
      <Callout.Root>
        <Callout.Icon>
          <InfoCircledIcon />
        </Callout.Icon>
        <Callout.Text>
          You will be redirected back in <b data-testid="timeout">{timeout}</b>{" "}
          second
          {timeout != 1 ? "s" : ""}.
        </Callout.Text>
      </Callout.Root>
    </AnimatedContainer>
  );
};
