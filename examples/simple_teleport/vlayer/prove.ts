import type { Address } from "viem";

import { testHelpers, prove } from "@vlayer/sdk";
import simpleTravelProver from "../out/SimpleTravelProver.sol/SimpleTravelProver";
import simpleTravelVerifier from "../out/SimpleTravelVerifier.sol/SimpleTravel";
import exampleNFT from "../out/ExampleNFT.sol/ExampleNFT";

const john = testHelpers.getTestAccount();

const deployProver = async () => {
  const prover: Address = await testHelpers.deployContract(
    simpleTravelProver,
    [],
  );

  return prover;
};

const deployVerifier = async (prover: Address) => {
  const rewardNFT: Address = await testHelpers.deployContract(exampleNFT, []);

  const verifier: Address = await testHelpers.deployContract(
    simpleTravelVerifier,
    [prover, rewardNFT],
  );

  return verifier;
};

console.log("Proving...");
const proverAddr = await deployProver();

const { proof, returnValue } = await prove(
  proverAddr,
  simpleTravelProver.abi,
  "crossChainBalanceOf",
  [john.address],
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
