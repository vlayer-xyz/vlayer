import { testHelpers } from "@vlayer/sdk";
import fs from "fs";
import path from "path";

import webProofProver from "../contracts/out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../contracts/out/WebProofVerifier.sol/WebProofVerifier";

const [prover, verifier] = await testHelpers.deployProverVerifier(
  webProofProver,
  webProofVerifier,
);

console.log(`Prover: ${prover}`);
console.log(`Verifier: ${verifier}`);

const envPath = path.resolve(__dirname, "../webapp/.env.development");
const envContent = `VITE_PROVER_ADDRESS=${prover}\nVITE_VERIFIER_ADDRESS=${verifier}\n`;

fs.writeFile(envPath, envContent, (err) => {
  if (err) {
    console.error("Error writing to .env.dev file:", err);
  } else {
    console.log("Successfully wrote to .env.dev file");
  }
});
