import { createVlayerClient } from "@vlayer/sdk";
import proverSpec from "../out/SimpleTeleportProver.sol/SimpleTeleportProver";
import verifierSpec from "../out/SimpleTeleportVerifier.sol/SimpleTeleportVerifier";
import whaleBadgeNFTSpec from "../out/WhaleBadgeNFT.sol/WhaleBadgeNFT";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
  waitForContractDeploy,
  waitForTransactionReceipt,
} from "@vlayer/sdk/config";

const config = getConfig();
const { ethClient, account } = await createContext(config);
const vlayer = createVlayerClient();

const deployWhaleBadgeHash = await ethClient.deployContract({
  abi: whaleBadgeNFTSpec.abi,
  bytecode: whaleBadgeNFTSpec.bytecode.object,
  account,
});

const whaleBadgeNFTAddress = await waitForContractDeploy({
  hash: deployWhaleBadgeHash,
});

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [],
  verifierArgs: [whaleBadgeNFTAddress],
});

console.log("Proving...");
const proofHash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "crossChainBalanceOf",
  args: [account.address],
});
const result = await vlayer.waitForProvingResult(proofHash);
console.log("Proof:", result[0]);
console.log("Verifying...");

const verificationHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "claim",
  args: result,
  account,
});

const receipt = await waitForTransactionReceipt({
  hash: verificationHash,
});

console.log(`Verification result: ${receipt.status}`);
