import * as React from "react";
import styles from "./CardContent.module.css";
import { VlayerBottomLogo } from "components/atoms/VlayerBottomLogo";
import { Box } from "@radix-ui/themes";

interface CardContentProps {
  imageSrc: string;
  title: string;
  description: string;
}

export const CardContent: React.FC<CardContentProps> = ({
  imageSrc,
  title,
  description,
}) => {
  return (
    <div className={styles.container}>
      <img loading="lazy" src={imageSrc} className={styles.image} />
      <div className={styles.title}>{title}</div>
      <div className={styles.description}>{description}</div>
      <Box className={styles.bottomLogo}>
        <VlayerBottomLogo />
      </Box>
    </div>
  );
};
