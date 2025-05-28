/// <reference types="bun" />

import { createVlayerClient } from "@vlayer/sdk";
import proverSpec from "../out/KrakenProver.sol/KrakenProver";
import verifierSpec from "../out/KrakenVerifier.sol/KrakenVerifier";
import {
  getConfig,
  createContext,
  deployVlayerContracts,
  writeEnvVariables,
} from "@vlayer/sdk/config";
import { spawn } from "child_process";

const URL_TO_PROVE = "https://api.kraken.com/0/public/Ticker?pair=ETHUSD";

const config = getConfig();
const { chain, ethClient, account, proverUrl, confirmations, notaryUrl } =
  createContext(config);

if (!account) {
  throw new Error(
    "No account found make sure EXAMPLES_TEST_PRIVATE_KEY is set in your environment variables",
  );
}

const vlayer = createVlayerClient({
  url: proverUrl,
  token: config.token,
});

async function generateWebProof() {
  console.log("⏳ Generating web proof...");
  const { stdout } = await runProcess("vlayer", [
    "web-proof-fetch",
    "--notary",
    String(notaryUrl),
    "--url",
    URL_TO_PROVE,
  ]);
  return stdout;
}

console.log("⏳ Deploying contracts...");

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [],
  verifierArgs: [],
});

await writeEnvVariables(".env", {
  VITE_PROVER_ADDRESS: prover,
  VITE_VERIFIER_ADDRESS: verifier,
});

console.log("✅ Contracts deployed", { prover, verifier });

const webProof = await generateWebProof();

console.log("⏳ Proving...");
const hash = await vlayer.prove({
  address: prover,
  functionName: "main",
  proverAbi: proverSpec.abi,
  args: [
    {
      webProofJson: String(webProof),
    },
  ],
  chainId: chain.id,
  gasLimit: config.gasLimit,
});
const result = await vlayer.waitForProvingResult({ hash });
const [proof, avgPrice] = result;
console.log("✅ Proof generated");

console.log("⏳ Verifying...");

// Workaround for viem estimating gas with `latest` block causing future block assumptions to fail on slower chains like mainnet/sepolia
const gas = await ethClient.estimateContractGas({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "verify",
  args: [proof, avgPrice],
  account,
  blockTag: "pending",
});

const txHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "verify",
  args: [proof, avgPrice],
  chain,
  account,
  gas,
});

await ethClient.waitForTransactionReceipt({
  hash: txHash,
  confirmations,
  retryCount: 60,
  retryDelay: 1000,
});

console.log("✅ Verified!");

function runProcess(
  cmd: string,
  args: string[],
): Promise<{ stdout: string; stderr: string }> {
  return new Promise((resolve, reject) => {
    const proc = spawn(cmd, args);
    let stdout = "";
    let stderr = "";
    proc.stdout.on("data", (data) => {
      stdout += data;
    });
    proc.stderr.on("data", (data) => {
      stderr += data;
    });
    proc.on("close", (code) => {
      if (code === 0) {
        resolve({ stdout, stderr });
      } else {
        reject(new Error(`Process failed: ${stderr}`));
      }
    });
    proc.on("error", (err) => {
      reject(err);
    });
  });
}
