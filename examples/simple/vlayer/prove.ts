import type { Address } from "viem";

import { testHelpers, prove } from "@vlayer/sdk";
import simpleProver from "../out/SimpleProver.sol/SimpleProver.json";

console.log("Deploying prover")
let prover: Address = await testHelpers.deployContract(simpleProver);
console.log(`Prover has been deployed on ${prover} address`);

console.log("Proving...");
let response = await prove(prover, simpleProver, 'sum', [1, 2]);
console.log("Response:")
console.log(response);
