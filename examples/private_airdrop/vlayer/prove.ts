import type { Address, Account } from "viem";

import { testHelpers, prove, createTestClient } from "@vlayer/sdk";
import exampleToken from "../out/ExampleToken.sol/ExampleToken.json";
import privateAirdropProver from "../out/PrivateAirdropProver.sol/PrivateAirdropProver.json";

const client = createTestClient();

const deployContracts = async (account: Account) => {
  console.log("Deploying prover...")
  let exampleErc20: Address = await testHelpers.deployContract(exampleToken, [[account.address]]);
  let proverAddress: Address = await testHelpers.deployContract(privateAirdropProver, [exampleErc20]);
  console.log(`Prover has been deployed on ${proverAddress} address`);

  return proverAddress;
}

const generateTestSignature = async (account: Account) => {
  const signature = await client.signMessage({ 
    account,
    message: 'I own ExampleToken and I want to privately claim my airdrop',
  })

  return signature;
}

const generateProof = async (prover: Address, tokenOwner: Account) => {
  const signature = await generateTestSignature(tokenOwner);

  let response = await prove(prover, privateAirdropProver, 'main', [tokenOwner.address, signature]);
  console.log("Response:", response)
}

const tokenOwner = testHelpers.getTestAccount();
const proverAddress = await deployContracts(tokenOwner);
await generateProof(proverAddress, tokenOwner);