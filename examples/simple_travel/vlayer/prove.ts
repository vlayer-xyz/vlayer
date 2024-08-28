import type { Address } from "viem";

import { testHelpers, prove } from "@vlayer/sdk";
import simpleTravelProver from "../out/SimpleTravelProver.sol/SimpleTravelProver.json";
import exampleToken from "../out/ExampleToken.sol/ExampleToken.json";

const john = testHelpers.getTestAccount();

console.log("Deploying example erc20 token on anvil 2");
const tokenB: Address = await testHelpers.deployContract(exampleToken, [[john.address]], testHelpers.chainIds[1]);
console.log(`Token has been deployed on ${tokenB} address`);

console.log("Deploying prover and example token on anvil 1");
const tokenA: Address = await testHelpers.deployContract(exampleToken, [[john.address]]);
console.log(`Token has been deployed on ${tokenA} address`);
const prover: Address = await testHelpers.deployContract(simpleTravelProver, [[tokenA, tokenB], [testHelpers.chainIds[0], testHelpers.chainIds[1]]]);
console.log(`Prover has been deployed on ${prover} address`);

console.log("Proving...");
const response = await prove(prover, simpleTravelProver, 'proveMultiChainOwnership', [john.address]);
console.log("Response:", response)
