import { type Address, erc20Abi } from "viem";
import assert from "node:assert";

import { testHelpers, prove } from "@vlayer/sdk";
import nftOwnershipProver from "../out/NftOwnershipProver.sol/NftOwnershipProver";
import airdropVerifier from "../out/AirdropVerifier.sol/Airdrop";

const [prover, verifier] = await testHelpers.deployProverVerifier(
  nftOwnershipProver,
  airdropVerifier,
);

console.log("Proving...");
const sender = testHelpers.getTestAccount().address;
const { proof, returnValue: [claimAddress] } = await prove(
  prover,
  nftOwnershipProver.abi,
  "main",
  [sender],
);
console.log("Proof:");
console.log(proof);
assert.equal(claimAddress, sender);

console.log("Balance before claim:", await getBalance(verifier, claimAddress));

console.log("Verifying...");
await testHelpers.writeContract(verifier, airdropVerifier.abi, "claim", [
  proof,
  claimAddress,
]);

const balance = await getBalance(verifier, claimAddress);
console.log("Balance after:", balance);
assert.equal(balance, 1000n);

async function getBalance(verifierAddress: Address, account: Address) {
  const tokenAddress = await testHelpers.call(
    airdropVerifier.abi,
    verifierAddress,
    "TOKEN",
  );
  return testHelpers.call(erc20Abi, tokenAddress, "balanceOf", [account]);
}
