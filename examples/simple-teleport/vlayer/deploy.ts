import proverSpec from "../out/SimpleTeleportProver.sol/SimpleTeleportProver";
import verifierSpec from "../out/SimpleTeleportVerifier.sol/SimpleTeleportVerifier";
import whaleBadgeNFTSpec from "../out/WhaleBadgeNFT.sol/WhaleBadgeNFT";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
  waitForContractDeploy,
  writeEnvVariables,
} from "@vlayer/sdk/config";
import { loadFixtures } from "./loadFixtures";
import { getTeleportConfig } from "./constants";

const config = getConfig();
const teleportConfig = getTeleportConfig(config.chainName);

if (config.chainName === "anvil") {
  await loadFixtures();
}

const { ethClient, account } = createContext(config);

if (!account) {
  throw new Error(
    "No account found make sure EXAMPLES_TEST_PRIVATE_KEY is set in your environment variables",
  );
}

console.log("⏳ Deploying helper contracts...");
const deployWhaleBadgeHash = await ethClient.deployContract({
  abi: whaleBadgeNFTSpec.abi,
  bytecode: whaleBadgeNFTSpec.bytecode.object,
  account,
});

const whaleBadgeNFTAddress = await waitForContractDeploy({
  client: ethClient,
  hash: deployWhaleBadgeHash,
});

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [],
  verifierArgs: [whaleBadgeNFTAddress],
});

const tokensToCheck: {
  addr: Address;
  chainId: string;
  blockNumber: string;
  balance: string;
}[] = (teleportConfig.prover.erc20Addresses.split(",") || []).map(
  (addr, i) => ({
    addr: addr as Address,
    chainId: teleportConfig.prover.erc20ChainIds.split(",")[i],
    blockNumber: teleportConfig.prover.erc20BlockNumbers.split(",")[i],
    balance: "0",
  }),
);

await writeEnvVariables(".env", {
  VITE_PROVER_ADDRESS: prover,
  VITE_VERIFIER_ADDRESS: verifier,
  VITE_CHAIN_NAME: config.chainName,
  VITE_PROVER_URL: config.proverUrl,
  VITE_PRIVATE_KEY: config.privateKey,
  VITE_VLAYER_API_TOKEN: config.token,
  VITE_TOKENS_TO_CHECK: `"${JSON.stringify(tokensToCheck)}"`,
  VITE_DEFAULT_TOKEN_HOLDER: teleportConfig.tokenHolder,
  VITE_GAS_LIMIT: config.gasLimit,
});
