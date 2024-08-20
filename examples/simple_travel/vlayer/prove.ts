import { type Address } from "viem";

import { testHelpers, prove } from "vlayer-sdk";
import { mainnet } from "viem/chains";
import otherChainContractSpec from "../out/OtherChainContract.sol/OtherChainContract.json";
import simpleTravelProver from "../out/SimpleTravelProver.sol/SimpleTravelProver.json";

console.log("Deploying a contract on mainnet chain");
let otherChainContract: Address = await testHelpers.deployContract(otherChainContractSpec, [], mainnet.id);
console.log(`Contract has been deployed on ${otherChainContract} address`);

console.log("Deploying prover on sepolia chain");
let prover: Address = await testHelpers.deployContract(simpleTravelProver);
console.log(`Prover has been deployed on ${prover} address`);

console.log("Proving...");
let response = await prove(prover, simpleTravelProver, 'aroundTheWorld', []);
console.log("Response:", response)
