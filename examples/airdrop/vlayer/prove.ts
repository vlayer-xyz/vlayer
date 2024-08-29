import {type Address, erc20Abi} from "viem";

import {testHelpers, completeProof} from "@vlayer/sdk";
import nftOwnershipProver from "../out/NftOwnershipProver.sol/NftOwnershipProver";
import airdropVerifier from "../out/AirdropVerifier.sol/Airdrop";
import assert from "node:assert";

console.log("Deploying contracts")
const prover: Address = await testHelpers.deployContract(nftOwnershipProver);
console.log(`Prover has been deployed on ${prover} address`);
const verifier = await testHelpers.deployContract(airdropVerifier, [prover]);
console.log(`Verifier has been deployed on ${verifier} address`);

console.log("Proving...");
const sender = testHelpers.getTestAccount().address;
const {
  proof,
  returnValue: claimAddress
} = await completeProof(prover, nftOwnershipProver.abi, "main", [sender]);
console.log("Proof:")
console.log(proof);

assert.equal(claimAddress, sender);

console.log("Verifying...")
await testHelpers.writeContract(verifier, airdropVerifier.abi, "claim", [proof, claimAddress]);

const balance = await getBalance(verifier, claimAddress)
console.log("Balance:", balance);
assert.equal(balance, 1000n);

async function getBalance(verifierAddress: Address, account: Address) {
  const tokenAddress = await testHelpers.call(airdropVerifier.abi, verifierAddress, "TOKEN");
  return testHelpers.call(erc20Abi, tokenAddress, "balanceOf", [account]);
}