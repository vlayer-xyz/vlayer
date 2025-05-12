import { createVlayerClient } from "@vlayer/sdk";
import proverSpec from "../out/AverageBalance.sol/AverageBalance";
import verifierSpec from "../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
  waitForTransactionReceipt,
} from "@vlayer/sdk/config";
import { getStartEndBlock } from "./helpers";
import { loadFixtures } from "./loadFixtures";
import { getTimeTravelConfig } from "./constants";

const config = getConfig();
const timeTravelConfig = getTimeTravelConfig(config.chainName);

if (config.chainName === "anvil") {
  await loadFixtures();
}

const { ethClient, account, proverUrl } = createContext(config);

if (!account) {
  throw new Error(
    "No account found make sure EXAMPLES_TEST_PRIVATE_KEY is set in your environment variables",
  );
}

const { startBlock, endBlock } = await getStartEndBlock({
  config,
  timeTravelConfig,
});

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [
    timeTravelConfig.usdcTokenAddr,
    startBlock,
    endBlock,
    timeTravelConfig.prover.step,
  ],
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
  args: [timeTravelConfig.tokenOwner],
  chainId: ethClient.chain.id,
  gasLimit: config.gasLimit,
});

console.log("Waiting for proving result: ");

const result = await vlayer.waitForProvingResult({ hash: provingHash });

console.log("Proof:", result[0]);
console.log("Verifying...");

// Workaround for viem estimating gas with `latest` block causing future block assumptions to fail on slower chains like mainnet/sepolia
const gas = await ethClient.estimateContractGas({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "claim",
  args: result,
  account,
  blockTag: "pending",
});

const verificationHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "claim",
  args: result,
  account,
  gas,
});

const receipt = await waitForTransactionReceipt({
  client: ethClient,
  hash: verificationHash,
});

console.log(`Verification result: ${receipt.status}`);
