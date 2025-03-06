import { createVlayerClient } from "@vlayer/sdk";
import proverSpec from "../out/AverageBalance.sol/AverageBalance";
import verifierSpec from "../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
  waitForTransactionReceipt,
} from "@vlayer/sdk/config";
import { type Address } from "viem";

const config = getConfig();
const { ethClient, account, proverUrl } = await createContext(config);

declare const process: {
  env: {
    PROVER_START_BLOCK: bigint;
    PROVER_END_BLOCK: bigint | "latest";
    PROVER_TRAVEL_RANGE: bigint;
    PROVER_ERC20_CONTRACT_ADDR: string;
    PROVER_ERC20_HOLDER_ADDR: string;
    PROVER_STEP: bigint;
  };
};

const useLatestBlock = process.env.PROVER_END_BLOCK === "latest";
const endBlock = useLatestBlock
  ? await ethClient.getBlockNumber()
  : BigInt(process.env.PROVER_END_BLOCK);

const startBlock = useLatestBlock
  ? endBlock - BigInt(process.env.PROVER_TRAVEL_RANGE)
  : BigInt(process.env.PROVER_START_BLOCK);

const tokenOwner = process.env.PROVER_ERC20_HOLDER_ADDR as Address;
const usdcTokenAddr = process.env.PROVER_ERC20_CONTRACT_ADDR as Address;

const step = BigInt(process.env.PROVER_STEP);

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [usdcTokenAddr, startBlock, endBlock, step],
  verifierArgs: [],
});

const vlayer = createVlayerClient({
  url: proverUrl,
});

const token = config.token;
const provingHash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "averageBalanceOf",
  args: [tokenOwner],
  chainId: ethClient.chain.id,
  token,
});

console.log("Waiting for proving result: ");

const result = await vlayer.waitForProvingResult({ hash: provingHash, token });

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
