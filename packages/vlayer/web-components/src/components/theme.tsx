import { Theme } from "@radix-ui/themes";
import type { PropsWithChildren } from "react";
import "./theme.css";
export const VlayerTheme = function (props: PropsWithChildren) {
  return <Theme accentColor="violet">{props.children}</Theme>;
};
