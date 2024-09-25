import type { Address } from "viem";

import { sepolia } from "viem/chains";

import { testHelpers, prove } from "@vlayer/sdk";
import averageBalance from "../out/AverageBalance.sol/AverageBalance";
import averageBalanceVerifier from "../out/AverageBalanceVerifier.sol/AverageBalanceVerifier";
import hodlerBadge from "../out/HodlerBadgeNFT.sol/HodlerBadgeNFT";

const chainId = sepolia.id;
const usdcTokenAddr = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238";
const tokenOwner = "0x6dBe810e3314546009bD6e1B29f9031211CdA5d2";
const startBlock = 6639262;
const endBlock = 6709262;
const step = 9000;

const deployProver = async () => {
  const prover: Address = await testHelpers.deployContract(averageBalance, [
    chainId,
    usdcTokenAddr,
    startBlock,
    endBlock,
    step,
  ]);

  return prover;
};

const deployVerifier = async (prover: Address) => {
  const rewardNFT: Address = await testHelpers.deployContract(hodlerBadge, []);

  const verifier: Address = await testHelpers.deployContract(
    averageBalanceVerifier,
    [prover, rewardNFT],
  );

  return verifier;
};

console.log("Proving...");
const proverAddr = await deployProver();

const { proof, returnValue } = await prove({
  abi: averageBalance.abi,
  functionName: "averageBalanceOf",
  args: [tokenOwner],
  prover: proverAddr,
});
console.log("Response:", proof, returnValue);

const verifierAddr = await deployVerifier(proverAddr);
const receipt = await testHelpers.writeContract(
  verifierAddr,
  averageBalanceVerifier.abi,
  "claim",
  [proof, ...returnValue],
);
console.log(`Verification result: ${receipt.status}`);
