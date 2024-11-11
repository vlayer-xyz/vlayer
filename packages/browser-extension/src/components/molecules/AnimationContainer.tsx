import { motion, Variants, AnimatePresence } from "framer-motion";
import { collapseAnimation } from "components/framer";
import React, { PropsWithChildren } from "react";

type AnimatedContainerProps = PropsWithChildren<{
  animation?: Variants;
  isVisible: boolean;
}>;

export const AnimatedContainer = ({
  children,
  animation = collapseAnimation,
  isVisible,
  ...props
}: AnimatedContainerProps) => (
  <AnimatePresence>
    {isVisible && (
      <motion.div {...animation} {...props}>
        {children}
      </motion.div>
    )}
  </AnimatePresence>
);
