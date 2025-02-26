import { z } from "zod";
import { envSchema } from "./utils/schema";
import { envToConfig } from "src/config/utils/envToConfig";

const transformedEnvSchema = envSchema.transform(envToConfig);
export type VlayerContextConfig = z.infer<typeof transformedEnvSchema>;
