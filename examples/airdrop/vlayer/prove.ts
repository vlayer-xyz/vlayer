import type {Address} from "viem";

import {testHelpers, prove} from "@vlayer/sdk";
import nftOwnershipProver from "../out/NftOwnershipProver.sol/NftOwnershipProver.json";

console.log("Deploying prover")
const prover: Address = await testHelpers.deployContract(nftOwnershipProver);
console.log(`Prover has been deployed on ${prover} address`);

console.log("Proving...");
const response = await prove(prover, nftOwnershipProver, "main", [testHelpers.getTestAccount().address]);
console.log("Response:")
console.log(response);
