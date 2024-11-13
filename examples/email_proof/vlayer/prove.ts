import fs from "fs";
import { createVlayerClient, preverifyEmail } from "@vlayer/sdk";
import {
  getConfig,
  createContext,
  deployVlayerContracts,
} from "@vlayer/sdk/config";

import proverSpec from "../out/EmailProver.sol/EmailProver";
import verifierSpec from "../out/EmailProofVerifier.sol/EmailProofVerifier";
import { foundry } from "viem/chains";

const mimeEmail = fs.readFileSync("./testdata/vlayer_welcome.eml").toString();
const unverifiedEmail = await preverifyEmail(mimeEmail);

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
});

const config = getConfig();
const { chain, ethClient, account } = await createContext(config);

console.log("Proving...");

const vlayer = createVlayerClient();
const hash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "main",
  chainId: foundry.id,
  args: [unverifiedEmail],
});
const result = await vlayer.waitForProvingResult(hash);
console.log("Proof:", result[0]);

console.log("Verifying...");

await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "verify",
  args: result,
  chain,
  account: account,
});

console.log("Verified!");
