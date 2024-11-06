import fs from "fs";
import { testHelpers, createVlayerClient, preverifyEmail } from "@vlayer/sdk";
import emailProofProver from "../out/EmailDomainProver.sol/EmailDomainProver";
import emailProofVerifier from "../out/EmailProofVerifier.sol/EmailDomainVerifier";

const mimeEmail = fs.readFileSync("./testdata/verify_vlayer.eml").toString();

const prover = await testHelpers.deployContract(emailProofProver, [
  "@vlayer.xyz",
]);

const verifier = await testHelpers.deployContract(emailProofVerifier, [
  prover,
  "vlayer badge",
  "VL",
]);
const john = testHelpers.getTestAccount();

console.log("Proving...");
const vlayer = createVlayerClient();
const hash = await vlayer.prove({
  address: prover,
  proverAbi: emailProofProver.abi,
  functionName: "main",
  args: [await preverifyEmail(mimeEmail), john.address],
});
const result = await vlayer.waitForProvingResult(hash);
console.log("Proof:", result[0]);

console.log("Verifying...");
await testHelpers.writeContract(
  verifier,
  emailProofVerifier.abi,
  "verify",
  result,
);
console.log("Verified!");
