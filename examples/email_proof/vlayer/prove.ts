import fs from "fs";
import { testHelpers, prove, enrichEmail } from "@vlayer/sdk";
import emailProofProver from "../out/EmailProver.sol/EmailProver";
import emailProofVerifier from "../out/EmailProofVerifier.sol/EmailProofVerifier";

const mimeEmail = fs.readFileSync("../testdata/test_email.txt").toString();

const unverifiedEmail = await enrichEmail(mimeEmail);

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
