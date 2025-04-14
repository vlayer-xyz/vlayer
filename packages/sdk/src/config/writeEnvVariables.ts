import fs from "fs";
import dotenv from "dotenv";
import debug from "debug";
import { filterOverrides } from "./filterOverrides";
import { type Overrides } from "./types";

const log = debug("vlayer:config");

export const writeEnvVariables = async (
  envPath: string,
  overrides: Overrides,
) => {
  fs.appendFileSync(envPath, "");
  const envFile = Bun.file(envPath);
  let envContent = await envFile.text();

  if (!envContent) {
    envContent = "";
  }

  const newEnvs = Object.assign(
    dotenv.parse(envContent),
    filterOverrides(overrides),
  );

  const envLines = Object.entries(newEnvs)
    .map(([key, value]) => `${key}=${value}`)
    .join("\n");

  await Bun.write(envPath, envLines);

  log(`Successfully updated the ${envPath} with: `, overrides);
};
