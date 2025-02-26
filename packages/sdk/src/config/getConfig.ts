
import type {  VlayerContextConfig } from "./types";
import { envToConfig } from "src/config/utils/envToConfig";
import { envSchema } from "./utils/schema";
import { dotEnvFlowConfig } from "src/config/utils/dotEnvConfig";
import { EnvValidationError } from "./utils/error";

<<<<<<< HEAD
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
    shouldDeployVerifierRouter,
    ...unchangedKeys
  } = keysToCamelCase(config);

  return {
    ...unchangedKeys,
    privateKey: examplesTestPrivateKey as Hex,
    token: vlayerApiToken,
    deployConfig: {
      shouldRedeployVerifierRouter: shouldDeployVerifierRouter ?? false,
    },
  };
};

const stringBoolean = z.enum(["true", "false"]).transform((x) => x === "true");

const envSchema = z.object({
  VLAYER_ENV: z.enum(POSSIBLE_VLAYER_ENVS),
  CHAIN_NAME: z.string(),
  PROVER_URL: z.string().url(),
  JSON_RPC_URL: z.string().url(),
  DNS_SERVICE_URL: z.string().url().optional(),
  L2_JSON_RPC_URL: z.string().url().optional(),
  EXAMPLES_TEST_PRIVATE_KEY: z
    .string()
    .startsWith("0x")
    .length(66)
    .regex(/^0x[0-9a-fA-F]{64}$/),
  VLAYER_API_TOKEN: z.string().optional(),
  SHOULD_DEPLOY_VERIFIER_ROUTER: stringBoolean.optional(),
});

export const getConfig = (override: Partial<EnvConfig> = {}): EnvConfig => {
=======
export const getConfig = (override: Partial<VlayerContextConfig> = {}): VlayerContextConfig => {
>>>>>>> ae6e1769 (Reorganize config)
  dotEnvFlowConfig();

  const validationResult = envSchema
    .transform(envToConfig)
    .safeParse(process.env);

  if (!validationResult.success) {
    throw new EnvValidationError(validationResult);
  }

  return {
    ...validationResult.data,
    ...override,
  };
};
