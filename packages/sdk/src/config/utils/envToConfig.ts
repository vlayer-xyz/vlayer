import type { Hex } from "viem";
import type { envSchema } from "../schema";
import { keysToCamelCase } from "./camelCase";
import type { z } from "zod";
import { match } from "ts-pattern";
import {
  DEFAULT_TESTNET_VGAS_LIMIT,
  DEFAULT_MAINNET_VGAS_LIMIT,
} from "../../constants";

export const envToConfig = (config: z.infer<typeof envSchema>) => {
  const {
    examplesTestPrivateKey,
    vlayerApiToken,
    shouldDeployVerifierRouter,
    vgasLimit,
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
      vgasLimit ??
      match(vlayerEnv)
        .with("testnet", "dev", () => DEFAULT_TESTNET_VGAS_LIMIT)
        .with("mainnet", () => DEFAULT_MAINNET_VGAS_LIMIT)
        .exhaustive(),
  };
};
