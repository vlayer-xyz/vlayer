import type { Hex } from "viem";
import type { envSchema } from "../schema";
import { keysToCamelCase } from "./camelCase";
import type { z } from "zod";

export const envToConfig = (config: z.infer<typeof envSchema>) => {
  const {
    examplesTestPrivateKey,
    vlayerApiToken,
    shouldDeployVerifierRouter,
    ...unchangedKeys
  } = keysToCamelCase(config);

  return {
    ...unchangedKeys,
    privateKey: examplesTestPrivateKey as Hex,
    token: vlayerApiToken ?? "",
    deployConfig: {
      shouldRedeployVerifierRouter: shouldDeployVerifierRouter ?? false,
    },
  };
};
