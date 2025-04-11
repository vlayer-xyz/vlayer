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

const URL_TO_PROVE = "https://api.kraken.com/0/public/Ticker?pair=ETHUSD";

const config = getConfig();
const { chain, ethClient, account, proverUrl, confirmations, notaryUrl } =
  createContext(config);

const vlayer = createVlayerClient({
  url: proverUrl,
  token: config.token,
});

async function generateWebProof() {
  console.log("⏳ Generating web proof...");
  const result =
    await Bun.$`vlayer web-proof-fetch --notary ${notaryUrl} --url ${URL_TO_PROVE}`;
  return result.stdout.toString();
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
      webProofJson: webProof.toString(),
    },
  ],
  chainId: chain.id,
});
const result = await vlayer.waitForProvingResult({ hash });
const [proof, avgPrice] = result;
console.log("✅ Proof generated");

console.log("⏳ Verifying...");
const txHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "verify",
  args: [proof, avgPrice],
  chain,
  account,
});

await ethClient.waitForTransactionReceipt({
  hash: txHash,
  confirmations,
  retryCount: 60,
  retryDelay: 1000,
});

console.log("✅ Verified!");
