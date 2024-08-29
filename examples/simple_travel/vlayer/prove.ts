import type { Address } from "viem";

import { testHelpers, prove, createTestClient } from "@vlayer/sdk";
import simpleTravelProver from "../out/SimpleTravelProver.sol/SimpleTravelProver";
import exampleToken from "../out/ExampleToken.sol/ExampleToken";
import simpleTravelVerifier from "../out/SimpleTravelVerifier.sol/SimpleTravel";
import exampleNFT from "../out/ExampleNFT.sol/ExampleNFT";

const john = testHelpers.getTestAccount();

const deployTestTokens = async (
  tester: Address,
  chainA: number,
  chainB: number,
) => {
  console.log("Deploying example erc20 token on searate chains");
  const tokenA: Address = await testHelpers.deployContract(
    exampleToken,
    [tester, 10_000_000_000],
    chainA,
  );
  const tokenB: Address = await testHelpers.deployContract(
    exampleToken,
    [tester, 10_000_000_000],
    chainB,
  );
  const rewardNFT: Address = await testHelpers.deployContract(
    exampleNFT,
    [],
    chainA,
  );

  return [tokenA, tokenB, rewardNFT];
};

const deployProver = async (
  tokens: Address[],
  chainIds: number[],
  blockNumbers: number[],
) => {
  const prover: Address = await testHelpers.deployContract(simpleTravelProver, [
    tokens,
    chainIds,
    blockNumbers,
  ]);

  return prover;
};

const deployVerifier = async (prover: Address, rewardNFT: Address) => {
  const verifier: Address = await testHelpers.deployContract(
    simpleTravelVerifier,
    [prover, rewardNFT],
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
const [tokenA, tokenB, rewardNFT] = await deployTestTokens(
  john.address,
  chainA,
  chainB,
);

console.log("Proving...");
const [blockNumberA, blockNumberB] = await getCurrentBlockNumbers();
const proverAddr = await deployProver(
  [tokenA, tokenB],
  [chainA, chainB],
  [blockNumberA, blockNumberB],
);

const proverParams = [
  john.address,
  [
    [tokenA, chainA, blockNumberA],
    [tokenB, chainB, blockNumberB],
  ],
];

const { proof, returnValue } = await prove(
  proverAddr,
  simpleTravelProver.abi,
  "multichainBalanceOf",
  proverParams,
);
console.log("Response:", proof, returnValue);

const verifierAddr = await deployVerifier(proverAddr, rewardNFT);
const receipt = await testHelpers.writeContract(
  verifierAddr,
  simpleTravelVerifier.abi,
  "claim",
  [proof, ...returnValue],
);
console.log(`Verification result: ${receipt.status}`);
