import type { Address, Account } from "viem";
import assert from "node:assert";

import { testHelpers, createTestClient, prove } from "@vlayer/sdk";
import exampleToken from "../out/ExampleToken.sol/ExampleToken";
import privateAirdropProver from "../out/PrivateAirdropProver.sol/PrivateAirdropProver";
import privateAirdropVerifier from "../out/PrivateAirdropVerifier.sol/PrivateAirdropVerifier";

const client = createTestClient();

const deployContracts = async (account: Account) => {
  const sender = (await client.getAddresses())[0];
  const exampleErc20: Address = await testHelpers.deployContract(exampleToken, [
    [account.address, sender],
  ]);

  const [prover, verifier] = await testHelpers.deployProverVerifier(
    privateAirdropProver,
    privateAirdropVerifier,
    {
      prover: [exampleErc20],
      verifier: [exampleErc20],
    },
  );

  await transferTokens(
    exampleErc20,
    verifier,
    await testHelpers.call(exampleToken.abi, exampleErc20, "balanceOf", [
      sender,
    ]),
  );

  return [prover, verifier, exampleErc20];
};

const transferTokens = async (token: Address, to: Address, amount: bigint) => {
  await testHelpers.writeContract(token, exampleToken.abi, "transfer", [
    to,
    amount,
  ]);
};

const generateTestSignature = async (account: Account) => {
  const signature = await client.signMessage({
    account,
    message: "I own ExampleToken and I want to privately claim my airdrop",
  });

  return signature;
};

const generateProof = async (prover: Address, tokenOwner: Account) => {
  const signature = await generateTestSignature(tokenOwner);

  const { proof, returnValue, ok } = await prove(
    prover,
    privateAirdropProver.abi,
    "main",
    [tokenOwner.address, signature],
  );
  assert(ok, "Proving failed");
  console.log("Proof:", proof);

  return { proof, returnValue };
};

const tokenOwner = testHelpers.getTestAccount();
const [proverAddress, verifierAddress, token] =
  await deployContracts(tokenOwner);
const {
  proof,
  returnValue: [account, nullifier],
} = await generateProof(proverAddress, tokenOwner);

const balanceBefore = await testHelpers.call(
  exampleToken.abi,
  token,
  "balanceOf",
  [account],
);
console.log("Balance before:", balanceBefore);

console.log("Verifying...");
await testHelpers.writeContract(
  verifierAddress,
  privateAirdropVerifier.abi,
  "claim",
  [proof, account, nullifier],
);

const balanceAfter = await testHelpers.call(
  exampleToken.abi,
  token,
  "balanceOf",
  [account],
);
console.log("Balance after:", balanceAfter);
assert.equal(balanceAfter - balanceBefore, 1000n);
