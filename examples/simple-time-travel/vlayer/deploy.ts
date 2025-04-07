import proverSpec from "../out/AverageBalance.sol/AverageBalance";
import verifierSpec from "../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";
import {
  deployVlayerContracts,
  getConfig,
  writeEnvVariables,
} from "@vlayer/sdk/config";
import { getStartEndBlock } from "./helpers";
import { loadFixtures } from "./loadFixtures";
import { getChainConfig } from "./constants";
const config = getConfig();
const chainConfig = getChainConfig(config.chainName);

if (config.chainName === "anvil") {
  await loadFixtures();
}

const { startBlock, endBlock } = await getStartEndBlock({
  config,
  chainConfig,
});

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [
    chainConfig.usdcTokenAddr,
    startBlock,
    endBlock,
    chainConfig.prover.step,
  ],
  verifierArgs: [],
});

writeEnvVariables(".env", {
  VITE_PROVER_ADDRESS: prover,
  VITE_VERIFIER_ADDRESS: verifier,
  VITE_CHAIN_NAME: config.chainName,
  VITE_PROVER_URL: config.proverUrl,
  VITE_PRIVATE_KEY: config.privateKey,
  VITE_VLAYER_API_TOKEN: config.token,
  VITE_PROVER_ERC20_HOLDER_ADDR: chainConfig.tokenOwner,
});
