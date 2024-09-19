import { testHelpers } from "@vlayer/sdk";
import Bun from "bun";
import path from "node:path";
import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";

const [prover, verifier] = await testHelpers.deployProverVerifier(
  webProofProver,
  webProofVerifier,
);

const envPath = path.resolve(__dirname, ".env.development");

try {
  const envFile = Bun.file(envPath);
  let envContent = await envFile.text();

  if (!envContent) {
    envContent = "";
  }

  envContent = `${envContent}\n`;

  const proverRegex = /^VITE_PROVER_ADDRESS=.*/m;
  const verifierRegex = /^VITE_VERIFIER_ADDRESS=.*/m;

  if (proverRegex.test(envContent)) {
    envContent = envContent.replace(
      proverRegex,
      `VITE_PROVER_ADDRESS=${prover}`,
    );
  } else {
    envContent += `VITE_PROVER_ADDRESS=${prover}\n`;
  }

  if (verifierRegex.test(envContent)) {
    envContent = envContent.replace(
      verifierRegex,
      `VITE_VERIFIER_ADDRESS=${verifier}`,
    );
  } else {
    envContent += `VITE_VERIFIER_ADDRESS=${verifier}\n`;
  }

  await Bun.write(envPath, envContent);
  console.log("Successfully updated the .env.development file");
} catch (err) {
  console.error("Error updating the .env.development file:", err);
}
