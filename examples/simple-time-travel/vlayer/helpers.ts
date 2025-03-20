import { env } from "./env";
import { getConfig, createContext } from "@vlayer/sdk/config";

const config = getConfig();
const { ethClient } = await createContext(config);

const useLatestBlock = env.PROVER_END_BLOCK === "latest";
const latestBlock = await ethClient.getBlockNumber();

let startBlock: bigint;
const endBlock = useLatestBlock ? latestBlock : env.PROVER_END_BLOCK;

if (env.PROVER_TRAVEL_RANGE) {
  startBlock = latestBlock - env.PROVER_TRAVEL_RANGE;
} else {
  startBlock = env.PROVER_START_BLOCK;
}

export { startBlock, endBlock };
