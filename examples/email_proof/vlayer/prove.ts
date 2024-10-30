import fs from "fs";
import { testHelpers, createVlayerClient, preverifyEmail } from "@vlayer/sdk";
import emailProofProver from "../out/EmailProver.sol/EmailProver";
import emailProofVerifier from "../out/EmailProofVerifier.sol/EmailProofVerifier";

const mimeEmail = fs.readFileSync("./testdata/vlayer_welcome.eml").toString();

const unverifiedEmail = await preverifyEmail(mimeEmail);

const [prover, verifier] = await testHelpers.deployProverVerifier(
  emailProofProver,
  emailProofVerifier,
);

console.log("Proving...");
const vlayer = createVlayerClient();
const { hash } = await vlayer.prove({
  address: prover,
  proverAbi: emailProofProver.abi,
  functionName: "main",
  args: [unverifiedEmail],
});
const result = await vlayer.waitForProvingResult({ hash });
console.log("Proof:", result[0]);

console.log("Verifying...");
await testHelpers.writeContract(
  verifier,
  emailProofVerifier.abi,
  "verify",
  result,
);
console.log("Verified!");
