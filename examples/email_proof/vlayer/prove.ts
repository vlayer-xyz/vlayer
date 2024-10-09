import fs from "fs";
import { testHelpers, createVlayerClient, preverifyEmail } from "@vlayer/sdk";
import emailProofProver from "../out/EmailProver.sol/EmailProver";
import emailProofVerifier from "../out/EmailProofVerifier.sol/EmailProofVerifier";

const mimeEmail = fs
  .readFileSync("./testdata/real_signed_email.eml")
  .toString();

const unverifiedEmail = await preverifyEmail(mimeEmail);

const [prover, verifier] = await testHelpers.deployProverVerifier(
  emailProofProver,
  emailProofVerifier,
);

console.log("Proving...");
const vlayer = createVlayerClient();
const {
  proof,
  result: [result],
} = await vlayer.prove({
  address: prover,
  proverAbi: emailProofProver.abi,
  functionName: "main",
  args: [unverifiedEmail],
});
console.log("Proof:", proof);

console.log("Verifying...");
await testHelpers.writeContract(verifier, emailProofVerifier.abi, "verify", [
  proof,
  result,
]);
console.log("Verified!");
