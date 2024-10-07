import React, { PropsWithChildren } from "react";
import styles from "./Circle.module.css";
import { motion, AnimatePresence } from "framer-motion";
export const Circle = ({
  isSolid,
  isDisabled,
  children,
}: PropsWithChildren<{
  isSolid: boolean;
  isDisabled: boolean;
}>) => {
  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        transition={{ ease: "easeOut", duration: 2 }}
        className={`${isDisabled ? styles.disabled : ""} ${isSolid ? styles.solidCircle : ""} ${styles.circle} `}
      >
        {children}
      </motion.div>
    </AnimatePresence>
  );
};
