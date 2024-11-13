import proverSpec from "../out/WebProofProver.sol/WebProofProver";
import verifierSpec from "../out/WebProofVerifier.sol/WebProofVerifier";
import { deploy, writeEnvVariables } from "@vlayer/sdk/config";

const { prover, verifier } = await deploy({ proverSpec, verifierSpec });

writeEnvVariables(".env", {
  VITE_PROVER_ADDRESS: prover,
  VITE_VERIFIER_ADDRESS: verifier,
});
