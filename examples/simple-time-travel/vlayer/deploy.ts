import proverSpec from "../out/AverageBalance.sol/AverageBalance";
import verifierSpec from "../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
  writeEnvVariables,
} from "@vlayer/sdk/config";
import { type Address } from "viem";
import { env } from "./env";

const config = getConfig();
const { ethClient } = await createContext(config);

const useLatestBlock = env.PROVER_END_BLOCK === "latest";
const latestBlock = await ethClient.getBlockNumber();
const endBlock = useLatestBlock ? latestBlock : env.PROVER_END_BLOCK;

let startBlock;

if (env.PROVER_TRAVEL_RANGE) {
  startBlock = latestBlock - env.PROVER_TRAVEL_RANGE;
} else {
  startBlock = env.PROVER_START_BLOCK;
}

if (!startBlock) {
  throw new Error("Start block is required");
}

const usdcTokenAddr = env.PROVER_ERC20_CONTRACT_ADDR as Address;

const step = env.PROVER_STEP;

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [usdcTokenAddr, startBlock, endBlock, step],
  verifierArgs: [],
});

writeEnvVariables(".env", {
  VITE_PROVER_ADDRESS: prover,
  VITE_VERIFIER_ADDRESS: verifier,
  VITE_CHAIN_NAME: config.chainName,
  VITE_PROVER_URL: config.proverUrl,
  VITE_PRIVATE_KEY: config.privateKey,
  VITE_VLAYER_API_TOKEN: config.token,
  VITE_USE_WINDOW_ETHEREUM_TRANSPORT: env.USE_WINDOW_ETHEREUM_TRANSPORT || "",
});
