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
import { type Address } from "viem";

const config = getConfig();
const { chain, ethClient, account, proverUrl, confirmations } =
  createContext(config);
const vlayer = createVlayerClient({
  url: proverUrl,
  token: config.token,
});

const deployWhaleBadgeHash = await ethClient.deployContract({
  abi: whaleBadgeNFTSpec.abi,
  bytecode: whaleBadgeNFTSpec.bytecode.object,
  account,
});

const whaleBadgeNFTAddress = await waitForContractDeploy({
  client: ethClient,
  hash: deployWhaleBadgeHash,
});

const tokensToCheck: { addr: Address; chainId: bigint; blockNumber: bigint }[] =
  (process.env.PROVER_ERC20_ADDRESSES?.split(",") || []).map((addr, i) => ({
    addr: addr as Address,
    chainId: BigInt(process.env.PROVER_ERC20_CHAIN_IDS?.split(",")[i]),
    blockNumber: BigInt(process.env.PROVER_ERC20_BLOCK_NUMBERS?.split(",")[i]),
  }));

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
  args: [process.env.TOKEN_HOLDER as Address, tokensToCheck],
  chainId: chain.id,
});
const result = await vlayer.waitForProvingResult({ hash: proofHash });
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
