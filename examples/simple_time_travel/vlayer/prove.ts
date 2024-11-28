import { createVlayerClient } from "@vlayer/sdk";
import proverSpec from "../out/AverageBalance.sol/AverageBalance";
import verifierSpec from "../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
  waitForTransactionReceipt,
} from "@vlayer/sdk/config";
import { $ } from "bun";
import { type Address } from "viem";

const config = getConfig();
const { ethClient, account, proverUrl } = await createContext(config);

let tokenOwner: Address;
let usdcTokenAddr: Address;
let startBlock: bigint;
let endBlock: bigint;
let step: bigint;

if (process.env.VLAYER_ENV === "dev") {
  await $`forge script --chain anvil scripts/AnvilSetup.s.sol:AnvilSetup --rpc-url ${config.jsonRpcUrl} --broadcast --private-key ${config.privateKey}`;
  tokenOwner = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"; // Owner of the USDC token at anvil
  usdcTokenAddr = "0x5FbDB2315678afecb367f032d93F642f64180aa3"; // USDC at anvil
  startBlock = 1n;
  endBlock = 40n;
  step = 10n;
} else {
  tokenOwner = "0xE6b08c02Dbf3a0a4D3763136285B85A9B492E391"; // Owner of the USDC token at OP Sepolia
  usdcTokenAddr = "0x5fd84259d66Cd46123540766Be93DFE6D43130D7"; // Test USDC at OP Sepolia
  startBlock = 17915294n;
  endBlock = 17985294n;
  step = 9000n;
}

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [usdcTokenAddr, startBlock, endBlock, step],
  verifierArgs: [],
});

const vlayer = createVlayerClient({
  url: proverUrl,
});

const provingHash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "averageBalanceOf",
  args: [tokenOwner],
  chainId: ethClient.chain.id,
});

console.log("Waiting for proving result: ");

const result = await vlayer.waitForProvingResult(provingHash);

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
  hash: verificationHash,
});

console.log(`Verification result: ${receipt.status}`);
