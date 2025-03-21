import { env } from "./env";
import { createContext, type VlayerContextConfig } from "@vlayer/sdk/config";

const getStartEndBlock = async (config: VlayerContextConfig) => {
  if (env.PROVER_END_BLOCK === "latest") {
    const { ethClient } = await createContext(config);
    const latestBlock = await ethClient.getBlockNumber();

    if (!env.PROVER_TRAVEL_RANGE) {
      throw new Error(
        "PROVER_TRAVEL_RANGE must be set if PROVER_END_BLOCK is set to 'latest'",
      );
    }
    return {
      startBlock: latestBlock - env.PROVER_TRAVEL_RANGE,
      endBlock: latestBlock,
    };
  }

  return {
    startBlock: env.PROVER_START_BLOCK,
    endBlock: env.PROVER_END_BLOCK,
  };
};

export { getStartEndBlock };
