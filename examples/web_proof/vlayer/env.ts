//
import * as dotenvflow from "dotenv-flow";
const ensureVlayerEnv = () => {
  if (!process.env.VLAYER_ENV) {
    throw new Error("VLAYER_ENV is not set");
  }
};

/*
 * Configuration function to load environment variables using dotenv-flow
 * see: https://www.npmjs.com/package/dotenv-flow
 * dotenv-flow loads the environment variables in the following priority (from lowest to highest):
 *
 * 1. `.env.defaults` - Loaded first, providing default values for environment variables.
 *    This is the base set of defaults for the environment.
 * 2. `.env` - Loaded after `.env.defaults`, providing additional or overriding configuration.
 *    This may come in handy when migrating from dotenv (where it is strongly advised against
 *    committing the `.env` file to VCS) and you already have a `.env` file used to store your local values.
 * 3. `.env.local` - Loaded after `.env`, used for local overrides (applies to all environments).
 * 4. `.env.<VLAYER_ENV>` - Loaded based on the `NODE_ENV` or `VLAYER_ENV` value (e.g., `.env.production`).
 * 5. `.env.<VLAYER_ENV>.local` - Loaded last, overrides settings in the environment-specific `.env.<VLAYER_ENV>`
 *    file (e.g., `.env.production.local` for production-specific local settings).
 * 6. env vars - Loaded last, with the highest priority. Any environment variables set manually (e.g., via
 *    `process.env`) will override the settings from all the files listed above.
 */

export const config = () => {
  ensureVlayerEnv();
  dotenvflow.config({
    node_env: process.env.VLAYER_ENV,
  });
};
