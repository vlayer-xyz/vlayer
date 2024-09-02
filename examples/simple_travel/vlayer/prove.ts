import type { Address } from "viem";

import { testHelpers, prove } from "@vlayer/sdk";
import otherChainContractSpec from "../out/OtherChainContract.sol/OtherChainContract";
import simpleTravelProver from "../out/SimpleTravelProver.sol/SimpleTravelProver";
import simpleTravelVerifier from "../out/SimpleTravelVerifier.sol/SimpleTravel";
import { testChainId2 } from "../../../packages/vlayer/sdk/src/api/helpers";

const [prover, verifier] = await testHelpers.deployProverVerifier(
  simpleTravelProver,
  simpleTravelVerifier,
);

console.log("Deploying a contract on anvil 2");
const otherChainContract: Address = await testHelpers.deployContract(
  otherChainContractSpec,
  [],
  testChainId2,
);
console.log(`Contract has been deployed on ${otherChainContract} address`);

console.log("Proving...");
const { proof, returnValue } = await prove(
  prover,
  simpleTravelProver.abi,
  "aroundTheWorld",
  [],
);
console.log("Proof:", proof);

console.log("Verifying...");
await testHelpers.writeContract(verifier, simpleTravelVerifier.abi, "verify", [
  proof,
  returnValue,
]);
console.log("Verified!");
