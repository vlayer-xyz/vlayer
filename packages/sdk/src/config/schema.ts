import { z } from "zod";
import { envToConfig } from "./utils/envToConfig";

export const POSSIBLE_VLAYER_ENVS = ["mainnet", "testnet", "dev"] as const;
export const POSSIBLE_CLIENT_AUTH_MODES = ["wallet", "envPrivateKey"] as const;
export const stringBoolean = z
  .enum(["true", "false"])
  .transform((x) => x === "true");

export const envSchema = z.object({
  CHAIN_NAME: z.string(),
  EXAMPLES_TEST_PRIVATE_KEY: z.string().regex(/^0x[0-9a-fA-F]{64}$/),
  CLIENT_AUTH_MODE: z.enum(POSSIBLE_CLIENT_AUTH_MODES).optional(),
  JSON_RPC_URL: z.string().url(),
  PROVER_URL: z.string().url(),
  VLAYER_ENV: z.enum(POSSIBLE_VLAYER_ENVS),
  DNS_SERVICE_URL: z.string().url().optional(),
  NOTARY_URL: z.string().url().optional(),
  SHOULD_DEPLOY_VERIFIER_ROUTER: stringBoolean.optional(),
  VLAYER_API_TOKEN: z.string().optional(),
  WS_PROXY_URL: z.string().url().optional(),
  GAS_LIMIT: z.string().regex(/^\d+$/).transform(Number).optional(),
});

export const configSchema = envSchema.transform(envToConfig);
