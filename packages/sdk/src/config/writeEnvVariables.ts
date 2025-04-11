import fs from "fs";
import dotenv from "dotenv";
import debug from "debug";

const log = debug("vlayer:config");

type Overrides = { [key: string]: string | undefined };
type DefinedOverrides = { [key: string]: string };

export function filterOverrides(overrides: Overrides): DefinedOverrides {
  const defined: DefinedOverrides = {};
  for (const key in overrides) {
    const value = overrides[key as keyof Overrides];
    if (value !== undefined) {
      defined[key] = value;
    }
  }
  return defined;
}

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
