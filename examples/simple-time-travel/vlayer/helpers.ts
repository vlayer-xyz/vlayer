import { TimeTravelConfig } from "./constants";
import { createContext, type VlayerContextConfig } from "@vlayer/sdk/config";

const getStartEndBlock = async ({
  config,
  timeTravelConfig,
}: {
  config: VlayerContextConfig;
  timeTravelConfig: TimeTravelConfig;
}) => {
  if (timeTravelConfig.prover.endBlock === "latest") {
    const { ethClient } = createContext(config);
    const latestBlock = await ethClient.getBlockNumber();

    return {
      startBlock: latestBlock - timeTravelConfig.prover.travelRange,
      endBlock: latestBlock,
    };
  }

  return {
    startBlock: timeTravelConfig.prover.startBlock,
    endBlock: timeTravelConfig.prover.endBlock,
  };
};

export { getStartEndBlock };
