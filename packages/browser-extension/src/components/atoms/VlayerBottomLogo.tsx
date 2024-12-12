import React, { FC } from "react";
import { Separator } from "@radix-ui/themes";
export const VlayerBottomLogo: FC = () => {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
      }}
    >
      <Separator size={"2"} style={{ marginBottom: "10px" }} />
      <img loading="lazy" src="/bottomlogo.svg" />
    </div>
  );
};
