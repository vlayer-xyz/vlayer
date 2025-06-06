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
  const [activeTab] = await browser.tabs.query({
    active: true,
    currentWindow: true,
  });

  if (!activeTab || !activeTab.id) {
    throw new Error("No active tab found");
  }

  await browser.tabs.update(activeTab.id, { url: link });
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
