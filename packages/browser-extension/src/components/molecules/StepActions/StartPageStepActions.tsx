import React, { FC } from "react";
import { Text, Grid } from "@radix-ui/themes";
import { StepStatus } from "constants/step";
import { Button } from "components/atoms";
import browser from "webextension-polyfill";
import { motion, AnimatePresence } from "framer-motion";
import sendMessageToServiceWorker from "lib/sendMessageToServiceWorker";
import { ExtensionMessageType } from "../../../web-proof-commons";

type StartPageStepActionProps = {
  isVisited: boolean;
  link: string;
  status: StepStatus;
  buttonText: string;
};

export const StartPageStepActions: FC<StartPageStepActionProps> = ({
  isVisited,
  link,
  status,
}) => {
  return (
    <AnimatePresence>
      {!isVisited && status == StepStatus.Current && (
        <Grid columns={"5"}>
          <motion.div>
            <Button
              data-testid="start-page-button"
              variant={"soft"}
              style={{
                gridColumn: "1 / 5",
                marginBottom: "1rem",
              }}
              // open app we gona take proof from in new tab
              onClick={async () => {
                const tab = await browser.tabs.create({
                  url: link,
                });
                await sendMessageToServiceWorker({
                  type: ExtensionMessageType.TabOpened,
                  tabId: tab.id!,
                });
              }}
            >
              <Text>Redirect</Text>
            </Button>
          </motion.div>
        </Grid>
      )}
    </AnimatePresence>
  );
};
