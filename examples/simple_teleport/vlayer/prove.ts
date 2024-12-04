import { createVlayerClient } from "@vlayer/sdk";
import proverSpec from "../out/SimpleTeleportProver.sol/SimpleTeleportProver";
import verifierSpec from "../out/SimpleTeleportVerifier.sol/SimpleTeleportVerifier";
import whaleBadgeNFTSpec from "../out/WhaleBadgeNFT.sol/WhaleBadgeNFT";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
  waitForContractDeploy,
} from "@vlayer/sdk/config";

const parseTokensEnv = () => {
  try {
    const tokensToCheck = [];
    const addresses = process.env.PROVER_ERC20_ADDRESSES?.split(",") || [];
    const chainIds = process.env.PROVER_ERC20_CHAIN_IDS?.split(",") || [];
    const blockNumbers =
      process.env.PROVER_ERC20_BLOCK_NUMBERS?.split(",") || [];

    for (let i = 0; i < addresses.length; i++) {
      tokensToCheck.push([addresses[i], chainIds[i], BigInt(blockNumbers[i])]);
    }

    return tokensToCheck;
  } catch (error) {
    console.error("Failed to parse ERC20_TOKENS_TO_CHECK:", error);
  }
};

const config = getConfig();
const { chain, ethClient, account, proverUrl, confirmations } =
  await createContext(config);
const vlayer = createVlayerClient({
  url: proverUrl,
});

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
  proverArgs: [parseTokensEnv() ?? []],
  verifierArgs: [whaleBadgeNFTAddress],
});

console.log("Proving...");
const proofHash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "crossChainBalanceOf",
  args: [account.address],
  chainId: chain.id,
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

const receipt = await ethClient.waitForTransactionReceipt({
  hash: verificationHash,
  confirmations,
  retryCount: 60,
  retryDelay: 1000,
});

console.log(`Verification result: ${receipt.status}`);
