import { TlsnProofContextProvider } from "hooks/useTlsnProver";
import { Grid, Theme } from "@radix-ui/themes";
import React, { FC } from "react";
import { SidePanelContainer } from "./SidePanelContent";
import styles from "./SidePanel.module.css";
export const SidePanel: FC = () => {
  return (
    <TlsnProofContextProvider>
      <Theme accentColor="violet">
        <Grid columns="10" gapY="4" top="16" className={styles.grid}>
          <div className={styles.container}>
            <SidePanelContainer />
          </div>
        </Grid>
      </Theme>
    </TlsnProofContextProvider>
  );
};
