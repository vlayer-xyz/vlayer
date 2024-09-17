import { testHelpers, prove } from "@vlayer/sdk";

import SimpleProver from "../out/SimpleProver.sol/SimpleProver";
import SimpleVerifier from "../out/SimpleVerifier.sol/SimpleVerifier";
import ExampleNftAbi from "../out/ExampleNFT.sol/ExampleNFT";
import ExampleToken from "../out/ExampleToken.sol/ExampleToken";
import assert from "node:assert";

const john = testHelpers.getTestAccount();
const exampleToken = await testHelpers.deployContract(ExampleToken, [
  john.address,
  10_000_000,
]);
const blockNumber = await testHelpers.client().getBlockNumber();
const rewardNFT = await testHelpers.deployContract(ExampleNftAbi, []);
const prover = await testHelpers.deployContract(SimpleProver, [
  exampleToken,
  blockNumber,
]);
const verifier = await testHelpers.deployContract(SimpleVerifier, [
  prover,
  rewardNFT,
]);

console.log("Proving...");
const { proof, returnValue, ok } = await prove(
  prover,
  SimpleProver.abi,
  "balance",
  [john.address],
);
assert(ok, "Proving failed");
console.log("Proof result:");
console.log(proof, returnValue);

const receipt = await testHelpers.writeContract(
  verifier,
  SimpleVerifier.abi,
  "claimWhale",
  [proof, ...returnValue],
);

console.log(`Verification result: ${receipt.status}`);
