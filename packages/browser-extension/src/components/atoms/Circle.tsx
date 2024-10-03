import React, { PropsWithChildren } from "react";
import styles from "./Circle.module.css";

export const Circle = ({
  isSolid,
  isDisabled,
  children,
}: PropsWithChildren<{
  isSolid: boolean;
  isDisabled: boolean;
}>) => {
  return (
    <div
      className={`${isDisabled ? styles.disabled : ""} ${isSolid ? styles.solidCircle : ""} ${styles.circle} `}
    >
      {children}
    </div>
  );
};
