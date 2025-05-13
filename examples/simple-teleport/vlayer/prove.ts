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
import { loadFixtures } from "./loadFixtures";
import { getTeleportConfig } from "./constants";

const config = getConfig();
const teleportConfig = getTeleportConfig(config.chainName);

if (config.chainName === "anvil") {
  await loadFixtures();
}

const { chain, ethClient, account, proverUrl, confirmations } =
  createContext(config);

if (!account) {
  throw new Error(
    "No account found make sure EXAMPLES_TEST_PRIVATE_KEY is set in your environment variables",
  );
}
const vlayer = createVlayerClient({
  url: proverUrl,
  token: config.token,
});
console.log("⏳ Deploying helper contracts...");
const deployWhaleBadgeHash = await ethClient.deployContract({
  abi: whaleBadgeNFTSpec.abi,
  bytecode: whaleBadgeNFTSpec.bytecode.object,
  account,
});

const whaleBadgeNFTAddress = await waitForContractDeploy({
  client: ethClient,
  hash: deployWhaleBadgeHash,
});

const tokensToCheck: {
  addr: Address;
  chainId: bigint;
  blockNumber: bigint;
  balance: bigint;
}[] = (teleportConfig.prover.erc20Addresses.split(",") || []).map(
  (addr, i) => ({
    addr: addr as Address,
    chainId: BigInt(teleportConfig.prover.erc20ChainIds.split(",")[i]),
    blockNumber: BigInt(teleportConfig.prover.erc20BlockNumbers.split(",")[i]),
    balance: BigInt(0),
  }),
);

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [],
  verifierArgs: [whaleBadgeNFTAddress],
});

const proofHash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "crossChainBalanceOf",
  args: [teleportConfig.tokenHolder, tokensToCheck],
  chainId: chain.id,
  gasLimit: config.gasLimit,
});
const result = await vlayer.waitForProvingResult({ hash: proofHash });
console.log("Proof:", result[0]);
console.log("⏳ Verifying...");

// Workaround for viem estimating gas with `latest` block causing future block assumptions to fail on slower chains like mainnet/sepolia
const gas = await ethClient.estimateContractGas({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "claim",
  args: result,
  account,
  blockTag: "pending",
});

const verificationHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "claim",
  args: result,
  account,
  gas,
});

const receipt = await ethClient.waitForTransactionReceipt({
  hash: verificationHash,
  confirmations,
  retryCount: 60,
  retryDelay: 1000,
});

console.log(`✅ Verification result: ${receipt.status}`);
