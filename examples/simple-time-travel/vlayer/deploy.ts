import proverSpec from "../out/AverageBalance.sol/AverageBalance";
import verifierSpec from "../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
  writeEnvVariables,
} from "@vlayer/sdk/config";
import { type Address } from "viem";

const config = getConfig();
const { ethClient } = await createContext(config);

declare const process: {
  env: {
    PROVER_START_BLOCK: bigint;
    PROVER_END_BLOCK: bigint | "latest";
    PROVER_TRAVEL_RANGE: bigint;
    PROVER_ERC20_CONTRACT_ADDR: string;
    PROVER_ERC20_HOLDER_ADDR: string;
    PROVER_STEP: bigint;
    USE_WINDOW_ETHEREUM_TRANSPORT: string;
  };
};

const useLatestBlock = process.env.PROVER_END_BLOCK === "latest";
const latestBlock = await ethClient.getBlockNumber();
const endBlock = useLatestBlock
  ? latestBlock
  : BigInt(process.env.PROVER_END_BLOCK);

let startBlock;

if (process.env.PROVER_TRAVEL_RANGE) {
  startBlock = latestBlock - BigInt(process.env.PROVER_TRAVEL_RANGE);
} else {
  startBlock = BigInt(process.env.PROVER_START_BLOCK);
}

const usdcTokenAddr = process.env.PROVER_ERC20_CONTRACT_ADDR as Address;

const step = BigInt(process.env.PROVER_STEP);

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
  VITE_USE_WINDOW_ETHEREUM_TRANSPORT:
    process.env.USE_WINDOW_ETHEREUM_TRANSPORT || "",
});
