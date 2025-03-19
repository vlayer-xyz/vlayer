import { z } from "zod";
import { type Address, isAddress } from "viem";
import { getConfig } from "@vlayer/sdk/config";

getConfig();
const addressSchema = z
  .string()
  .refine((val) => isAddress(val), {
    message: "Invalid Ethereum address format",
  })
  .transform((val) => val as Address);

const envSchema = z.object({
  PROVER_START_BLOCK: z
    .string()
    .optional()
    .transform((val) => (val ? BigInt(val) : undefined)),

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
  PROVER_ERC20_CONTRACT_ADDR: addressSchema,
  PROVER_ERC20_HOLDER_ADDR: addressSchema,
  PROVER_STEP: z.string().transform((val) => BigInt(val)),
  USE_WINDOW_ETHEREUM_TRANSPORT: z.enum(["true", "false"]),
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
