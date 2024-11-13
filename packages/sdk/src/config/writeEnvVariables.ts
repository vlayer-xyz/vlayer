import fs from "fs";
import dotenv from "dotenv";
export const writeEnvVariables = async (
  envPath: string,
  overrides: { [key: string]: string },
) => {
  fs.appendFileSync(envPath, "");
  const envFile = Bun.file(envPath);
  let envContent = await envFile.text();

  if (!envContent) {
    envContent = "";
  }

  const newEnvs = Object.assign(dotenv.parse(envContent), overrides);

  const envLines = Object.entries(newEnvs)
    .map(([key, value]) => `${key}=${value}`)
    .join("\n");

  await Bun.write(envPath, envLines);

  console.log(`Successfully updated the ${envPath} with: `, overrides);
};
