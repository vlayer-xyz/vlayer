import fs from "fs";
import { createVlayerClient, preverifyEmail } from "@vlayer/sdk";
import {
  getConfig,
  createContext,
  deployVlayerContracts,
} from "@vlayer/sdk/config";

import proverSpec from "../out/EmailProver.sol/EmailProver";
import verifierSpec from "../out/EmailProofVerifier.sol/EmailProofVerifier";

const mimeEmail = fs.readFileSync("./testdata/vlayer_welcome.eml").toString();
const unverifiedEmail = await preverifyEmail(mimeEmail);

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
});

const config = getConfig();
const { chain, ethClient, account, proverUrl, confirmations } =
  await createContext(config);

console.log("Proving...");

const vlayer = createVlayerClient({
  url: proverUrl,
});
const hash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "main",
  chainId: chain.id,
  args: [unverifiedEmail],
});
const result = await vlayer.waitForProvingResult(hash);
console.log("Proof:", result[0]);

console.log("Verifying...");

const txHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "verify",
  args: result,
  chain,
  account: account,
});

await ethClient.waitForTransactionReceipt({
  hash: txHash,
  confirmations,
  retryCount: 60,
  retryDelay: 1000,
});

console.log("Verified!");
