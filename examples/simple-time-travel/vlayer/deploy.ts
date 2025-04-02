import proverSpec from "../out/AverageBalance.sol/AverageBalance";
import verifierSpec from "../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";
import {
  deployVlayerContracts,
  getConfig,
  writeEnvVariables,
} from "@vlayer/sdk/config";
import { env } from "./env";
import { getStartEndBlock } from "./helpers";
const config = getConfig();

const usdcTokenAddr = env.PROVER_ERC20_CONTRACT_ADDR;

const step = env.PROVER_STEP;

const { startBlock, endBlock } = await getStartEndBlock(config);

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
  VITE_PROVER_ERC20_HOLDER_ADDR: process.env.PROVER_ERC20_HOLDER_ADDR,
});
