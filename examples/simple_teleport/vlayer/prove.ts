import type { Address } from "viem";

import { testHelpers, createVlayerClient } from "@vlayer/sdk";
import simpleTravelProver from "../out/SimpleTravelProver.sol/SimpleTravelProver";
import simpleTravelVerifier from "../out/SimpleTravelVerifier.sol/SimpleTravel";
import whaleBadgeNFT from "../out/WhaleBadgeNFT.sol/WhaleBadgeNFT";

const john = testHelpers.getTestAccount();

const deployProver = async () => {
  const prover: Address = await testHelpers.deployContract(
    simpleTravelProver,
    [],
  );

  return prover;
};

const deployVerifier = async (prover: Address) => {
  const rewardNFT: Address = await testHelpers.deployContract(
    whaleBadgeNFT,
    [],
  );

  const verifier: Address = await testHelpers.deployContract(
    simpleTravelVerifier,
    [prover, rewardNFT],
  );

  return verifier;
};

console.log("Proving...");
const proverAddr = await deployProver();
const vlayer = createVlayerClient();

const { hash } = vlayer.prove({
  address: proverAddr,
  proverAbi: simpleTravelProver.abi,
  functionName: "crossChainBalanceOf",
  args: [john.address],
});
const { proof, result } = await vlayer.waitForProvingResult({ hash });
console.log("Response:", proof, result);

const verifierAddr = await deployVerifier(proverAddr);
const receipt = await testHelpers.writeContract(
  verifierAddr,
  simpleTravelVerifier.abi,
  "claim",
  [proof, ...result],
);
console.log(`Verification result: ${receipt.status}`);
