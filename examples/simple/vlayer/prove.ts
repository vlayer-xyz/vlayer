import { testHelpers, prove } from "@vlayer/sdk";
import ProverAbi from "../out/SimpleProver.sol/SimpleProver";
import VerifierAbi from "../out/SimpleVerifier.sol/Simple";

const [prover, verifier] = await testHelpers.deployProverVerifier(
  ProverAbi,
  VerifierAbi,
);

console.log("Proving...");
const { proof, returnValue } = await prove(prover, ProverAbi.abi, "sum", [
  1n,
  2n,
]);
console.log("Proof result:");
console.log(proof, returnValue);

const receipt = await testHelpers.writeContract(
  verifier,
  VerifierAbi.abi,
  "updateSum",
  [proof, returnValue],
);

console.log(`Verification result: ${receipt.status}`);

const sumPost = await testHelpers.call(VerifierAbi.abi, verifier, "latestSum");
console.log(`Sum post: ${sumPost}`);
