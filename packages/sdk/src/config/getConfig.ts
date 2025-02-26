import type { VlayerContextConfig } from "./types";
import { envToConfig } from "./utils/envToConfig";
import { envSchema } from "./schema";
import { dotEnvFlowConfig } from "./utils/dotEnvConfig";
import { EnvValidationError } from "./errors";

export const getConfig = (
  override: Partial<VlayerContextConfig> = {},
): VlayerContextConfig => {
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
