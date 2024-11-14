import { optimismSepolia } from "viem/chains";
import { createVlayerClient } from "@vlayer/sdk";
import proverSpec from "../out/AverageBalance.sol/AverageBalance";
import verifierSpec from "../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
  waitForTransactionReceipt,
} from "@vlayer/sdk/config";

const tokenOwner = "0xE6b08c02Dbf3a0a4D3763136285B85A9B492E391"; // Owner of the USDC token at OP Sepolia
const usdcTokenAddr = "0x5fd84259d66Cd46123540766Be93DFE6D43130D7"; // Test USDC at OP Sepolia
const startBlock = 17915294n;
const endBlock = 17985294n;
const step = 9000n;

const config = getConfig();
const { ethClient, account } = await createContext(config);

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [usdcTokenAddr, startBlock, endBlock, step],
  verifierArgs: [],
});

const vlayer = createVlayerClient();

const provingHash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "averageBalanceOf",
  args: [tokenOwner],
  chainId: optimismSepolia.id,
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
