import dotenvflow from "dotenv-flow";
import type { Hex } from "viem";
import { z } from "zod";

import { type EnvConfig } from "./types";
import { keysToCamelCase } from "../utils/camelCase";

export class EnvValidationError extends Error {
  constructor(validationResult: z.SafeParseError<unknown>) {
    super(
      "Some environment variables are misconfigured:\n" +
        validationResult.error.errors
          .map((err) => `  - ${err.path.join(".")}: ${err.message}`)
          .join("\n"),
    );
    this.name = "EnvValidationError";
    Object.setPrototypeOf(this, EnvValidationError.prototype);
  }
}

const POSSIBLE_VLAYER_ENVS = ["testnet", "dev"] as const;

const dotEnvFlowConfig = () => {
  dotenvflow.config({
    node_env: process.env.VLAYER_ENV,
    default_node_env: "dev",
  });
};

const renameConfigKeys = (config: z.infer<typeof envSchema>) => {
  const {
    examplesTestPrivateKey,
    vlayerApiToken,
    shouldDeployFakeVerifier,
    ...unchangedKeys
  } = keysToCamelCase(config);

  return {
    ...unchangedKeys,
    privateKey: examplesTestPrivateKey as Hex,
    token: vlayerApiToken,
    deployConfig: {
      deployFakeVerifier: shouldDeployFakeVerifier ?? false,
    },
  };
};

const stringBoolean = z.enum(["true", "false"]).transform((x) => x === "true");

const envSchema = z.object({
  VLAYER_ENV: z.enum(POSSIBLE_VLAYER_ENVS),
  CHAIN_NAME: z.string(),
  PROVER_URL: z.string().url(),
  JSON_RPC_URL: z.string().url(),
  L2_JSON_RPC_URL: z.string().url().optional(),
  EXAMPLES_TEST_PRIVATE_KEY: z
    .string()
    .startsWith("0x")
    .length(66)
    .regex(/^0x[0-9a-fA-F]{64}$/),
  VLAYER_API_TOKEN: z.string().optional(),
  SHOULD_DEPLOY_FAKE_VERIFIER: stringBoolean.optional(),
});

export const getConfig = (override: Partial<EnvConfig> = {}): EnvConfig => {
  dotEnvFlowConfig();

  const validationResult = envSchema
    .transform(renameConfigKeys)
    .safeParse(process.env);

  if (!validationResult.success) {
    throw new EnvValidationError(validationResult);
  }

  return {
    ...validationResult.data,
    ...override,
  };
};
