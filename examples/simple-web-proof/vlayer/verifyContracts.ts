import { verifyContract } from "@vlayer/sdk/config";

const proverAddress = process.env.VITE_PROVER_ADDRESS as `0x${string}`;
const verifierAddress = process.env.VITE_VERIFIER_ADDRESS as `0x${string}`;

if (!proverAddress) {
  throw new Error("VITE_PROVER_ADDRESS is not set in environment variables");
}

if (!verifierAddress) {
  throw new Error("VITE_VERIFIER_ADDRESS is not set in environment variables");
}

// eslint-disable-next-line @typescript-eslint/no-unsafe-call
await verifyContract(proverAddress, "WebProofProver");
// eslint-disable-next-line @typescript-eslint/no-unsafe-call
await verifyContract(verifierAddress, "WebProofVerifier", `${proverAddress}`);
