import { testHelpers, createVlayerClient } from "@vlayer/sdk";

import SimpleProver from "../out/SimpleProver.sol/SimpleProver";
import SimpleVerifier from "../out/SimpleVerifier.sol/SimpleVerifier";
import ExampleNftAbi from "../out/ExampleNFT.sol/ExampleNFT";
import ExampleToken from "../out/ExampleToken.sol/ExampleToken";
import { foundry } from "viem/chains";
import { isAddress } from "viem";

const john = testHelpers.getTestAccount();
const exampleToken = await testHelpers.deployContract(ExampleToken, [
  john.address,
  10_000_000,
]);
const blockNumber = await testHelpers.createAnvilClient().getBlockNumber();
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
const hash = await vlayer.prove({
  address: prover,
  proverAbi: SimpleProver.abi,
  functionName: "balance",
  args: [john.address],
  chainId: foundry.id,
});
const result = await vlayer.waitForProvingResult(hash);
const [proof, owner, balance] = result;

if (!isAddress(owner)) {
  throw new Error(`${owner} is not a valid address`);
}

console.log("Proof result:");
console.log(result);

const receipt = await testHelpers.writeContract(
  verifier,
  SimpleVerifier.abi,
  "claimWhale",
  [proof, owner, balance],
);

console.log(`Verification result: ${receipt.status}`);
