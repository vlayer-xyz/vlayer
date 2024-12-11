// import { Steps } from "components/organisms";
import { TlsnProofContextProvider } from "hooks/useTlsnProver";
// import { HelpSection } from "components/organisms";
import { Grid, Theme } from "@radix-ui/themes";
import React, { FC } from "react";
import { SidePanelContent } from "./SidePanelContent";
export const SidePanel: FC = () => {
  return (
    <TlsnProofContextProvider>
      <Theme accentColor="violet">
        <Grid
          columns="10"
          gapY="4"
          top="16"
          style={{ paddingTop: "80px", height: "100vh" }}
          className="h-screen"
        >
          <div
            className="h-screen"
            style={{
              gridColumn: "span 10",
              fontFamily: "Sora",
            }}
          >
            <SidePanelContent />
          </div>
        </Grid>
      </Theme>
    </TlsnProofContextProvider>
  );
};
