import { Steps } from "components/organisms";
import { TlsnProofContextProvider } from "hooks/useTlsnProver";
import { HelpSection } from "components/organisms";
import { Grid, Theme } from "@radix-ui/themes";
import React, { FC } from "react";
export const NewSidePanel: FC = () => {
  return (
    <TlsnProofContextProvider>
      <Theme accentColor="violet">
        <Grid
          columns="10"
          gapY="4"
          top="16"
          style={{ marginTop: "80px", marginLeft: "10px" }}
        >
          <div style={{ gridColumn: "span 9", fontFamily: "Sora" }}>
            <Steps />
            <HelpSection />
          </div>
        </Grid>
      </Theme>
    </TlsnProofContextProvider>
  );
};
