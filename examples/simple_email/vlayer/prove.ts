import fs from "fs";
import { testHelpers, prove, preverifyEmail } from "@vlayer/sdk";
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
const { proof, returnValue } = await prove(
  prover,
  emailProofProver.abi,
  "main",
  [await preverifyEmail(mimeEmail), john.address],
);
console.log("Proof:", proof);

console.log("Verifying...");
await testHelpers.writeContract(verifier, emailProofVerifier.abi, "verify", [
  proof,
  ...returnValue,
]);
console.log("Verified!");
