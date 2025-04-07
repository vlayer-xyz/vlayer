import { ChainConfig } from "./constants";
import { createContext, type VlayerContextConfig } from "@vlayer/sdk/config";

const getStartEndBlock = async ({
  config,
  chainConfig,
}: {
  config: VlayerContextConfig;
  chainConfig: ChainConfig;
}) => {
  if (chainConfig.prover.endBlock === "latest") {
    const { ethClient } = await createContext(config);
    const latestBlock = await ethClient.getBlockNumber();

    return {
      startBlock: latestBlock - chainConfig.prover.travelRange,
      endBlock: latestBlock,
    };
  }

  return {
    startBlock: chainConfig.prover.startBlock,
    endBlock: chainConfig.prover.endBlock,
  };
};

export { getStartEndBlock };
