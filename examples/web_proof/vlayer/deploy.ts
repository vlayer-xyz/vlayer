import { testHelpers } from "@vlayer/sdk";
import fs from "node:fs";
import path from "node:path";

//@ts-expect-error - sol file is not a module
import webProofProver from "../out/WebProofProver.sol/WebProofProver";
//@ts-expect-error - sol file is not a module
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";

const [prover, verifier] = await testHelpers.deployProverVerifier(
  webProofProver,
  webProofVerifier,
);

const envPath = path.resolve(__dirname, ".env.development");
const envContent = `VITE_PROVER_ADDRESS=${prover}\nVITE_VERIFIER_ADDRESS=${verifier}\n`;

fs.writeFile(envPath, envContent, (err) => {
  if (err) {
    console.error("Error writing to .env.dev file:", err);
  } else {
    console.log("Successfully wrote to .env.dev file");
  }
});
