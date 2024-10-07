import React, { FC, PropsWithChildren } from "react";
import { Button as RadixButton, ButtonProps } from "@radix-ui/themes";
import style from "./Button.module.css";
export const Button: FC<PropsWithChildren<ButtonProps>> = (props) => {
  const propsWithDefaults = {
    size: "3",
    variant: "soft",
    ...props,
  } as const;
  return (
    <RadixButton {...propsWithDefaults} className={style.button}>
      {props.children}
    </RadixButton>
  );
};
