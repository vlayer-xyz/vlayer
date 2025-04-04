import { createVlayerClient } from "@vlayer/sdk";
import proverSpec from "../out/AverageBalance.sol/AverageBalance";
import verifierSpec from "../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
  waitForTransactionReceipt,
} from "@vlayer/sdk/config";
import { env } from "./env";
import { getStartEndBlock } from "./helpers";
import { loadFixtures } from "./loadFixtures";
import { getChainConfig } from "./constants";

const config = getConfig();
const chainConfig = getChainConfig(config.chainName);

if (config.chainName === "anvil") {
  await loadFixtures();
}

const { ethClient, account, proverUrl } = await createContext(config);

const { startBlock, endBlock } = await getStartEndBlock(config);

const step = env.PROVER_STEP;
const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [chainConfig.usdcTokenAddr, startBlock, endBlock, step],
  verifierArgs: [],
});

const vlayer = createVlayerClient({
  url: proverUrl,
  token: config.token,
});

const provingHash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "averageBalanceOf",
  args: [chainConfig.tokenOwner],
  chainId: ethClient.chain.id,
});

console.log("Waiting for proving result: ");

const result = await vlayer.waitForProvingResult({ hash: provingHash });

console.log("Proof:", result[0]);
console.log("Verifying...");

const verificationHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "claim",
  args: result,
  account,
});

const receipt = await waitForTransactionReceipt({
  client: ethClient,
  hash: verificationHash,
});

console.log(`Verification result: ${receipt.status}`);
