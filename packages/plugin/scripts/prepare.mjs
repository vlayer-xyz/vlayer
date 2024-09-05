import fs from "fs";
import { logger } from "./logger.js";
await $`ls`;
await $`git submodule update --init --recursive`;
await $`cd tlsn && git checkout tags/v0.1.0-alpha.5`;
logger.info("Submodules have been initialized");

await $`cd websockify && ./docker/build.sh`;
logger.info("Websockify has been built");

const configFilePath = "tlsn/notary-server/config/config.yaml";
let configContent = fs.readFileSync(configFilePath, "utf-8");
configContent = configContent.replace(
  /^(\s*)enabled:\s*true/m,
  "$1enabled: false",
);
fs.writeFileSync(configFilePath, configContent);
logger.info("TLS has been disabled in config.yaml");

await $`cd webapp && bun i`;
logger.info("Webapp dependencies have been installed");

await $`cd browser-plugin && bun i`;
logger.info("Plugin dependencies have been installed");
