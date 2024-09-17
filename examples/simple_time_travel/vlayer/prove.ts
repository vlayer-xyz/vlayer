import type { Address } from "viem";

import { sepolia } from "viem/chains";

import { testHelpers, prove } from "@vlayer/sdk";
import simpleTimeTravelProver from "../out/SimpleTimeTravelProver.sol/SimpleTimeTravelProver";
import simpleTimeTravelVerifier from "../out/SimpleTimeTravelVerifier.sol/SimpleTimeTravel";
import exampleNFT from "../out/ExampleNFT.sol/ExampleNFT";

const chainId = sepolia.id;
const usdcTokenAddr = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238";
const tokenOwner = "0x6dBe810e3314546009bD6e1B29f9031211CdA5d2";

const deployProver = async () => {
  const prover: Address = await testHelpers.deployContract(
    simpleTimeTravelProver,
    [chainId, usdcTokenAddr],
  );

  return prover;
};

const deployVerifier = async (prover: Address) => {
  const rewardNFT: Address = await testHelpers.deployContract(exampleNFT, []);

  const verifier: Address = await testHelpers.deployContract(
    simpleTimeTravelVerifier,
    [prover, rewardNFT],
  );

  return verifier;
};

console.log("Proving...");
const proverAddr = await deployProver();

const { proof, returnValue } = await prove(
  proverAddr,
  simpleTimeTravelProver.abi,
  "averageBalanceOf",
  [tokenOwner],
);
console.log("Response:", proof, returnValue);

const verifierAddr = await deployVerifier(proverAddr);
const receipt = await testHelpers.writeContract(
  verifierAddr,
  simpleTimeTravelVerifier.abi,
  "claim",
  [proof, ...returnValue],
);
console.log(`Verification result: ${receipt.status}`);
