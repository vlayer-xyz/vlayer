import React, { FC } from "react";
import styles from "./HelpSection.module.css";
import { Box, Flex, Text, Link, Separator, Heading } from "@radix-ui/themes";
import { VlayerBottomLogo } from "components/atoms/VlayerBottomLogo";

const LifebuoyIcon: FC = () => {
  return <img loading="lazy" src="/livebuoy.svg" alt="Help icon" />;
};

export const HelpSection: FC = () => {
  return (
    <Flex className={styles.container}>
      <Separator
        orientation="horizontal"
        size={"4"}
        className={styles.separator}
      />
      <Flex gap={"2"} align={"center"} justify={"start"} width={"100%"}>
        <LifebuoyIcon />
        <Heading size={"2"}>Having Trouble?</Heading>
      </Flex>
      <Box className={styles.spacer} />
      <Text size={"2"}>
        Feel free to&nbsp;
        <Link href="https://discord.gg/JS6whdessP" target="_blank">
          join our discord
        </Link>
        , if you need help through the process.
      </Text>
      <Box className={styles.spacer} />
      <VlayerBottomLogo />
    </Flex>
  );
};
