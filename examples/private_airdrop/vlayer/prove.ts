import { testHelpers, prove } from "vlayer-sdk";
import exampleToken from "../out/ExampleToken.sol/ExampleToken.json";
import privateAirdropProver from "../out/PrivateAirdropProver.sol/PrivateAirdropProver.json";

import type { Address, Account } from "viem";

const client = testHelpers.client();

const deployContracts = async (account: Account) => {
  console.log("Deploying prover")
  let exampleErc20: Address = await testHelpers.deployContract(exampleToken, [[account.address]]);
  let prover: Address = await testHelpers.deployContract(privateAirdropProver, [exampleErc20]);
  console.log(`Prover has been deployed on ${prover} address`);

  return { prover };
}

const generateTestSignature = async (account: Account) => {
  const signature = await client.signMessage({ 
    account,
    message: 'erc20 prover',
  })

  return signature;
}

const generateProof = async (prover: Address, tokenOwner: Account) => {
  const signature = await generateTestSignature(tokenOwner);
  let blockNo = Number(await testHelpers.client().getBlockNumber());
  console.log(`Running proving on ${blockNo} block number`);

  let response = await prove(prover, privateAirdropProver, 'main', [tokenOwner.address, signature], blockNo);
  console.log("Response:", response)
}

const tokenOwner = testHelpers.getTestAccount();
const { prover } = await deployContracts(tokenOwner);
await generateProof(prover, tokenOwner);