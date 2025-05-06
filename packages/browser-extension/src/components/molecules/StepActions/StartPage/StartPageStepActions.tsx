import React, { FC, useEffect } from "react";
import { Text, Grid } from "@radix-ui/themes";
import { StepStatus } from "constants/step";
import { Button } from "components/atoms";
import browser from "webextension-polyfill";
import { motion, AnimatePresence } from "framer-motion";
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
  const handleClick = () => {
    openApp(link).catch((error) => {
      console.error("Error during opening app:", error);
    });
  };

  useEffect(() => {
    if (!isVisited && status == StepStatus.Current) {
      handleClick();
    }
  }, [isVisited, status]);

  return (
    <AnimatePresence>
      {!isVisited && status == StepStatus.Current && (
        <Grid columns={"5"}>
          <motion.div>
            <Button
              color="violet"
              data-testid="start-page-button"
              variant={"soft"}
              style={{
                gridColumn: "1 / 5",
                marginBottom: "1rem",
              }}
              // open app we gona take proof from in new tab
              onClick={handleClick}
            >
              <Text>Redirect</Text>
            </Button>
          </motion.div>
        </Grid>
      )}
    </AnimatePresence>
  );
};
