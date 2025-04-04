import { z } from "zod";
import { getConfig } from "@vlayer/sdk/config";

getConfig();

const envSchema = z.object({
  PROVER_START_BLOCK: z
    .string()
    .optional()
    .transform((val) => (val ? BigInt(val) : BigInt(0))),

  PROVER_END_BLOCK: z.union([
    z.string().refine((val) => val === "latest", {
      message: "PROVER_END_BLOCK must be 'latest' or a valid number",
    }),
    z.string().transform((val) => BigInt(val)),
  ]),

  PROVER_TRAVEL_RANGE: z
    .string()
    .optional()
    .transform((val) => (val ? BigInt(val) : undefined)),
  PROVER_STEP: z.string().transform((val) => BigInt(val)),
});

type Env = z.infer<typeof envSchema>;

function parseEnv(): Env {
  try {
    return envSchema.parse(process.env);
  } catch (error) {
    console.error("Invalid environment variables:", error);
    process.exit(1);
  }
}

export const env = parseEnv();
