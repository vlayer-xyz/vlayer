import fs from "fs";
import { testHelpers, prove, preverifyEmail } from "@vlayer/sdk";
import emailProofProver from "../out/EmailProver.sol/EmailProver";
import emailProofVerifier from "../out/EmailProofVerifier.sol/EmailProofVerifier";

const mimeEmail = fs
  .readFileSync("../testdata/real_signed_email.eml")
  .toString();

const unverifiedEmail = await preverifyEmail(mimeEmail);

const [prover, verifier] = await testHelpers.deployProverVerifier(
  emailProofProver,
  emailProofVerifier,
);

console.log("Proving...");
const { proof, returnValue } = await prove(
  prover,
  emailProofProver.abi,
  "main",
  [unverifiedEmail],
);
console.log("Proof:", proof);

console.log("Verifying...");
await testHelpers.writeContract(verifier, emailProofVerifier.abi, "verify", [
  proof,
  returnValue,
]);
console.log("Verified!");
