import type { Address } from "viem";

import { testHelpers, createVlayerClient } from "@vlayer/sdk";
import simpleTeleportProver from "../out/SimpleTeleportProver.sol/SimpleTeleportProver";
import simpleTeleportVerifier from "../out/SimpleTeleportVerifier.sol/SimpleTeleportVerifier";
import whaleBadgeNFT from "../out/WhaleBadgeNFT.sol/WhaleBadgeNFT";

const john = testHelpers.getTestAccount().address;

const deployProver = async () => {
  const prover: Address = await testHelpers.deployContract(
    simpleTeleportProver,
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
    simpleTeleportVerifier,
    [prover, rewardNFT],
  );

  return verifier;
};

console.log("Proving...");
const proverAddr = await deployProver();
const vlayer = createVlayerClient();

const { hash } = await vlayer.prove({
  address: proverAddr,
  proverAbi: simpleTeleportProver.abi,
  functionName: "crossChainBalanceOf",
  args: [john],
});
const result = await vlayer.waitForProvingResult({ hash });
console.log("Response:", result);

const verifierAddr = await deployVerifier(proverAddr);
const receipt = await testHelpers.writeContract(
  verifierAddr,
  simpleTeleportVerifier.abi,
  "claim",
  result,
);
console.log(`Verification result: ${receipt.status}`);
