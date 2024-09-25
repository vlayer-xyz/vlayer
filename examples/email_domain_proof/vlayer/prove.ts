import fs from "fs";
import { testHelpers, prove } from "@vlayer/sdk";
import emailProofProver from "../out/EmailDomainProver.sol/EmailDomainProver";
import emailProofVerifier from "../out/EmailProofVerifier.sol/EmailDomainVerifier";

const mimeEmail = fs.readFileSync("../testdata/test_email.txt").toString();

const unverifiedEmail = { email: mimeEmail };

const prover = await testHelpers.deployContract(emailProofProver, [
  "football.example.com",
]);

const verifier = await testHelpers.deployContract(emailProofVerifier, [prover]);
const john = testHelpers.getTestAccount();

console.log("Proving...");
const { proof, returnValue } = await prove(
  prover,
  emailProofProver.abi,
  "main",
  [unverifiedEmail, john.address],
);
console.log("Proof:", proof);

console.log("Verifying...");
await testHelpers.writeContract(verifier, emailProofVerifier.abi, "verify", [
  proof,
  ...returnValue,
]);
console.log("Verified!");
