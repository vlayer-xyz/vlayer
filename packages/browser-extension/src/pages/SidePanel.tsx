import React from "react";
import { Button, Theme, Grid } from "@radix-ui/themes";
export default function SidePanel() {
  return (
    <Theme accentColor="violet">
      <Grid columns="8" gapY="4" top="16" style={{ marginTop: "80px" }}>
        <div style={{ gridColumn: "span 1" }}></div>
        <div style={{ gridColumn: "span 6" }}>
          <Grid columns="1" gapY="4">
            <Button variant="soft"> Go to page .. </Button>
            <Button> Make a proof </Button>
          </Grid>
        </div>
      </Grid>
    </Theme>
  );
}
