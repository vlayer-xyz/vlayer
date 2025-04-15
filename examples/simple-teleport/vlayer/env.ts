/// <reference types="node" />

import { z } from "zod";
import { getConfig } from "@vlayer/sdk/config";

getConfig();

const envSchema = z.object({
  L2_JSON_RPC_URL: z.string().url(),
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
