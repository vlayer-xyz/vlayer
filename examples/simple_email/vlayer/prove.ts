import fs from "fs";
import { createVlayerClient, preverifyEmail } from "@vlayer/sdk";
import proverSpec from "../out/EmailDomainProver.sol/EmailDomainProver";
import verifierSpec from "../out/EmailProofVerifier.sol/EmailDomainVerifier";
import { foundry } from "viem/chains";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
  waitForTransactionReceipt,
} from "@vlayer/sdk/config";

const mimeEmail = fs.readFileSync("./testdata/verify_vlayer.eml").toString();

const config = getConfig();

const { ethClient, account: john } = await createContext(config);

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: ["@vlayer.xyz"],
  verifierArgs: ["vlayer badge", "VL"],
});

console.log("Proving...");
const vlayer = createVlayerClient();
const hash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "main",
  chainId: foundry.id,
  args: [await preverifyEmail(mimeEmail), john.address],
});
const result = await vlayer.waitForProvingResult(hash);
console.log("Proof:", result[0]);

console.log("Verifying...");

const verificationHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "verify",
  args: result,
  account: john,
});

const receipt = await waitForTransactionReceipt({
  hash: verificationHash,
});

console.log(`Verification result: ${receipt.status}`);
