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

const config = getConfig();
const { ethClient, account, proverUrl } = await createContext(config);

const useLatestBlock = env.PROVER_END_BLOCK === "latest";
const latestBlock = await ethClient.getBlockNumber();
let endBlock;
if (useLatestBlock) {
  endBlock = latestBlock;
} else {
  endBlock = env.PROVER_END_BLOCK;
}

let startBlock;
if (env.PROVER_TRAVEL_RANGE) {
  startBlock = latestBlock - env.PROVER_TRAVEL_RANGE;
} else {
  startBlock = env.PROVER_START_BLOCK;
}

if (!startBlock) {
  throw new Error("Start block is required");
}

const tokenOwner = env.PROVER_ERC20_HOLDER_ADDR;
const usdcTokenAddr = env.PROVER_ERC20_CONTRACT_ADDR;

const step = env.PROVER_STEP;
const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [usdcTokenAddr, startBlock, endBlock, step],
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
  args: [tokenOwner],
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
