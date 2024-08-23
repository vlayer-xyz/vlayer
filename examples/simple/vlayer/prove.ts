import {helpers} from "vlayer-sdk";
import {type Address} from "viem";
import {call, writeContract} from "../../../packages/src/api/helpers";
import {completeProof} from "../../../packages/src/api/prover.ts";
import ProverAbi from "../out/SimpleProver.sol/SimpleProver";
import VerifierAbi from "../out/SimpleVerifier.sol/Simple";

const FUNCTION_NAME = 'sum'
const ARGS = [1n, 120n] as const

console.log("Deploying prover")
let prover: Address = await helpers.deployContract(ProverAbi);
let verifier: Address = await helpers.deployContract(VerifierAbi, [prover]);
console.log(`Prover has been deployed on ${prover} address`);
console.log(`Verifier has been deployed on ${verifier} address`);

console.log("Proving...");
let {proof, returnValue} = await completeProof(prover, ProverAbi.abi, FUNCTION_NAME, ARGS);
console.log("Proof result:")
console.log(proof, returnValue);

const receipt = await writeContract(verifier, VerifierAbi.abi, "updateSum", [proof, returnValue])

console.log(`Verification result: ${receipt.status}`);

let sumPost = await call(VerifierAbi.abi, verifier, "latestSum");
console.log(`Sum post: ${sumPost}`);
