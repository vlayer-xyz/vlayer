import type { Address } from "viem";

import { testHelpers, prove, createTestClient } from "@vlayer/sdk";
import simpleTravelProver from "../out/SimpleTravelProver.sol/SimpleTravelProver";
import exampleToken from "../out/ExampleToken.sol/ExampleToken";
import simpleTravelVerifier from "../out/SimpleTravelVerifier.sol/SimpleTravel";

const john = testHelpers.getTestAccount();

const deployTestTokens = async (
  tester: Address,
  chainA: number,
  chainB: number,
) => {
  console.log("Deploying example erc20 token on searate chains");
  const tokenA: Address = await testHelpers.deployContract(
    exampleToken,
    [[tester]],
    chainA,
  );
  const tokenB: Address = await testHelpers.deployContract(
    exampleToken,
    [[tester]],
    chainB,
  );

  return [tokenA, tokenB];
};

const deployProver = async () => {
  const prover: Address = await testHelpers.deployContract(
    simpleTravelProver,
    [],
  );

  return prover;
};

const deployVerifier = async (prover: Address) => {
  const verifier: Address = await testHelpers.deployContract(
    simpleTravelVerifier,
    [prover],
  );

  return verifier;
};

const getCurrentBlockNumbers = async () => {
  const clientA = await createTestClient(chainA);
  const blockNumberA = await clientA.getBlockNumber();
  const clientB = await createTestClient(chainB);
  const blockNumberB = await clientB.getBlockNumber();

  return [blockNumberA, blockNumberB];
};

const [chainA, chainB] = testHelpers.chainIds;
const [tokenA, tokenB] = await deployTestTokens(john.address, chainA, chainB);
const proverAddr = await deployProver();

const [blockNumberA, blockNumberB] = await getCurrentBlockNumbers();

const proverParams = [
  [tokenA, tokenB],
  john.address,
  [chainA, chainB],
  [blockNumberA, blockNumberB],
];

const { proof, returnValue } = await prove(
  proverAddr,
  simpleTravelProver.abi,
  "proveMultiChainOwnership",
  proverParams,
);
console.log("Response:", proof, returnValue);

const verifierAddr = await deployVerifier(proverAddr);
const receipt = await testHelpers.writeContract(
  verifierAddr,
  simpleTravelVerifier.abi,
  "claim",
  [proof, ...returnValue],
);

console.log(`Verification result: ${receipt.status}`);
