import type {Address} from "viem";

import { testHelpers, prove } from "@vlayer/sdk";
import nftOwnershipProver from "../out/NftOwnershipProver.sol/NftOwnershipProver.json";

console.log("Deploying prover")
let prover: Address = await testHelpers.deployContract(nftOwnershipProver);
console.log(`Prover has been deployed on ${prover} address`);

console.log("Proving...");
let response = await prove(prover, nftOwnershipProver, "main", ["0xaAa2DA255DF9Ee74C7075bCB6D81f97940908A5D"]);
console.log("Response:")
console.log(response);
