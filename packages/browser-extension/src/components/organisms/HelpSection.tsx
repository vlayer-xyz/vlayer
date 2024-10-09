import React, { FC } from "react";
import styles from "./HelpSection.module.css";
import { Box, Flex, Text, Link, Separator, Heading } from "@radix-ui/themes";

const LifebuoyIcon: FC = () => {
  return (
    <img
      loading="lazy"
      src="https://cdn.builder.io/api/v1/image/assets/TEMP/79782b9ae9b5b92b4a8a1a41370bd9b03b0cc6467894fa2d63221e7b47f9c81c?placeholderIfAbsent=true&apiKey=aea35748a39044d594404af7fb028825"
      alt="Help icon"
    />
  );
};

const VlayerLogo: FC = () => {
  return (
    <img
      loading="lazy"
      src="https://cdn.builder.io/api/v1/image/assets/TEMP/dfdbf09b8d39497695878264c38b56779ccef3608f24d7c893bc5db72f00ce54?placeholderIfAbsent=true&apiKey=aea35748a39044d594404af7fb028825"
      className={styles.logo}
    />
  );
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
        <Heading>Having Trouble?</Heading>
      </Flex>
      <Box className={styles.spacer} />
      <Text>
        Feel free to <Link href="https://discord.gg/JS6whdessP">contact us</Link>, if you need help
        through the process.
      </Text>
      <Box className={styles.spacer} />
      <Separator orientation="horizontal" size={"3"} />
      <Box className={styles.spacer} />
      <VlayerLogo />
    </Flex>
  );
};
