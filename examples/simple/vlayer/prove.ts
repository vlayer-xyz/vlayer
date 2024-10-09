import { testHelpers, createVlayerClient } from "@vlayer/sdk";

import SimpleProver from "../out/SimpleProver.sol/SimpleProver";
import SimpleVerifier from "../out/SimpleVerifier.sol/SimpleVerifier";
import ExampleNftAbi from "../out/ExampleNFT.sol/ExampleNFT";
import ExampleToken from "../out/ExampleToken.sol/ExampleToken";

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
const vlayer = createVlayerClient();
const { proof, result } = await vlayer.prove({
  address: prover,
  proverAbi: SimpleProver.abi,
  functionName: "balance",
  args: [john.address],
});
console.log("Proof result:");
console.log(proof, result);

const receipt = await testHelpers.writeContract(
  verifier,
  SimpleVerifier.abi,
  "claimWhale",
  [proof, ...result],
);

console.log(`Verification result: ${receipt.status}`);
