import type { Hex } from "viem";
import type { envSchema } from "../schema";
import { keysToCamelCase } from "./camelCase";
import type { z } from "zod";
import { match } from "ts-pattern";
import {
  DEFAULT_TESTNET_GAS_LIMIT,
  DEFAULT_MAINNET_GAS_LIMIT,
} from "../../constants";

export const envToConfig = (config: z.infer<typeof envSchema>) => {
  const {
    examplesTestPrivateKey,
    vlayerApiToken,
    shouldDeployVerifierRouter,
    gasLimit,
    vlayerEnv,
    ...unchangedKeys
  } = keysToCamelCase(config);

  return {
    ...unchangedKeys,
    vlayerEnv,
    privateKey: examplesTestPrivateKey as Hex,
    token: vlayerApiToken,
    deployConfig: {
      shouldRedeployVerifierRouter: shouldDeployVerifierRouter ?? false,
    },
    gasLimit:
      gasLimit ??
      match(vlayerEnv)
        .with("testnet", "dev", () => DEFAULT_TESTNET_GAS_LIMIT)
        .with("mainnet", () => DEFAULT_MAINNET_GAS_LIMIT)
        .exhaustive(),
  };
};
