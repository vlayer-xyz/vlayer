import React, { FC, useEffect } from "react";
import { StepStatus } from "constants/step";
import browser from "webextension-polyfill";

type RedirectStepActionProps = {
  isVisited: boolean;
  link: string;
  status: StepStatus;
  buttonText: string;
};

const redirect = async (link: string): Promise<void> => {
  const tabs = await browser.tabs.query({
    active: true,
    currentWindow: true,
  });
  await browser.tabs.update(tabs[0].id, { url: link });
};

export const RedirectStepActions: FC<RedirectStepActionProps> = ({
  isVisited,
  link,
  status,
}) => {
  useEffect(() => {
    if (!isVisited && status == StepStatus.Current) {
      redirect(link).catch((error) => {
        console.error("Error during redirecting:", error);
      });
    }
  }, [isVisited, link, status]);

  return <></>;
};
