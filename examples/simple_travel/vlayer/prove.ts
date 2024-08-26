import type { Address } from "viem";

import { testHelpers, prove } from "@vlayer/sdk";
import otherChainContractSpec from "../out/OtherChainContract.sol/OtherChainContract.json";
import simpleTravelProver from "../out/SimpleTravelProver.sol/SimpleTravelProver.json";
import { testChainId2 } from "../../../packages/vlayer/sdk/src/api/helpers";

console.log("Deploying prover on sepolia");
let prover: Address = await testHelpers.deployContract(simpleTravelProver);
console.log(`Prover has been deployed on ${prover} address`);

console.log("Deploying a contract on anvil 2");
let otherChainContract: Address = await testHelpers.deployContract(otherChainContractSpec, [], testChainId2);
console.log(`Contract has been deployed on ${otherChainContract} address`);

console.log("Proving...");
let response = await prove(prover, simpleTravelProver, 'aroundTheWorld', []);
console.log("Response:", response)
