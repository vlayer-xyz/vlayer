import { z } from "zod";

const POSSIBLE_VLAYER_ENVS = ["testnet", "dev"] as const;
const stringBoolean = z.enum(["true", "false"]).transform((x) => x === "true");

export const envSchema = z.object({
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
    SHOULD_DEPLOY_VERIFIER_ROUTER: stringBoolean.optional(),
    NOTARY_URL: z.string().url().optional(),
    WS_PROXY_URL: z.string().url().optional(),
    DNS_SERVICE_URL: z.string().url().optional(),
  });