import { z } from "zod";
import { envToConfig } from "./envToConfig";

export const POSSIBLE_VLAYER_ENVS = ["testnet", "dev"] as const;
export const stringBoolean = z
  .enum(["true", "false"])
  .transform((x) => x === "true");

export const envSchema = z.object({
  CHAIN_NAME: z.string(),
  EXAMPLES_TEST_PRIVATE_KEY: z.string().regex(/^0x[0-9a-fA-F]{64}$/),
  JSON_RPC_URL: z.string().url(),
  PROVER_URL: z.string().url(),
  VLAYER_ENV: z.enum(POSSIBLE_VLAYER_ENVS),
  DNS_SERVICE_URL: z.string().url().optional(),
  L2_JSON_RPC_URL: z.string().url().optional(),
  NOTARY_URL: z.string().url().optional(),
  SHOULD_DEPLOY_VERIFIER_ROUTER: stringBoolean.optional(),
  VLAYER_API_TOKEN: z.string().optional(),
  WS_PROXY_URL: z.string().url().optional(),
});

export const configSchema = envSchema.transform(envToConfig);
