import React, { FC, useEffect } from "react";
import { StepStatus } from "constants/step";
import browser from "webextension-polyfill";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";
import { ExtensionInternalMessageType } from "../../../../web-proof-commons";

type StartPageStepActionProps = {
  isVisited: boolean;
  link: string;
  status: StepStatus;
  buttonText: string;
};

const openApp = async (link: string): Promise<void> => {
  const tab = await browser.tabs.create({
    url: link,
  });
  await sendMessageToServiceWorker({
    type: ExtensionInternalMessageType.TabOpened,
    payload: {
      tabId: tab.id!,
    },
  });
};

export const StartPageStepActions: FC<StartPageStepActionProps> = ({
  isVisited,
  link,
  status,
}) => {
  useEffect(() => {
    if (!isVisited && status == StepStatus.Current) {
      openApp(link).catch((error) => {
        console.error("Error during opening app:", error);
      });
    }
  }, [isVisited, link, status]);

  return <></>;
};
