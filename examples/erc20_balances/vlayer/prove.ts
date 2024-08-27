import { type Address } from "viem";

import { testHelpers, prove } from "@vlayer/sdk";
import vToyken from "../out/VToyken.sol/VToyken.json";
import erc20Prover from "../out/ERC20Prover.sol/ERC20Prover.json";

const tokenOwners: Address[] =  ['0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC'];
const ARGS = [tokenOwners];

console.log("Deploying prover")
const token: Address = await testHelpers.deployContract(vToyken);
const prover: Address = await testHelpers.deployContract(erc20Prover, [token]);
console.log(`Prover has been deployed on ${prover} address`);


console.log("Proving...");
const response = await prove(prover, erc20Prover, 'prove', ARGS);
console.log("Response:")
console.log(response);
