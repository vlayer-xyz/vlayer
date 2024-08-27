import {testHelpers, completeProof} from "@vlayer/sdk";
import type { Address } from "viem";
import ProverAbi from "../out/SimpleProver.sol/SimpleProver";
import VerifierAbi from "../out/SimpleVerifier.sol/Simple";

const FUNCTION_NAME = 'sum'
const ARGS = [1n, 120n] as const

console.log("Deploying prover")
const prover: Address = await testHelpers.deployContract(ProverAbi);
const verifier: Address = await testHelpers.deployContract(VerifierAbi, [prover]);
console.log(`Prover has been deployed on ${prover} address`);
console.log(`Verifier has been deployed on ${verifier} address`);

console.log("Proving...");
const { proof, returnValue } = await completeProof(prover, ProverAbi.abi, FUNCTION_NAME, ARGS);
console.log("Proof result:")
console.log(proof, returnValue);

const receipt = await testHelpers.writeContract(verifier, VerifierAbi.abi, "updateSum", [proof, returnValue])

console.log(`Verification result: ${receipt.status}`);

const sumPost = await testHelpers.call(VerifierAbi.abi, verifier, "latestSum");
console.log(`Sum post: ${sumPost}`);
