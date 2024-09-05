import {testHelpers, prove} from "@vlayer/sdk";
import emailProofProver from "../out/EmailProver.sol/EmailProver";
import emailProofVerifier from "../out/EmailProofVerifier.sol/EmailProofVerifier";

const email = {mimeEmail: 'Mock email'};

const [prover, verifier] = await testHelpers.deployProverVerifier(
  emailProofProver,
  emailProofVerifier,
);

console.log("Proving...");
const {proof, returnValue} = await prove(prover, emailProofProver.abi, "main", [
  email
]);
console.log("Proof:", proof);

console.log("Verifying...");
await testHelpers.writeContract(verifier, emailProofVerifier.abi, "verify", [
  proof,
  returnValue,
]);
console.log("Verified!");
