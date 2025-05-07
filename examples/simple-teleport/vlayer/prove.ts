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
    "No account found make sure EXAMPLES_TEST_PRIVATE_KEY is set in your environment variables"
  );
}
const vlayer = createVlayerClient({
  url: proverUrl,
  token: config.token,
});
console.log("⏳ Deploying helper contracts...");

console.log("🧾 Using account:");
console.log("  Address:", account.address);

const chainId = await ethClient.getChainId?.(); // Optional chaining if not supported
console.log("🔗 Chain ID:", chainId || config.chainName);

console.log(
  "📦 Deploying contract with bytecode length:",
  whaleBadgeNFTSpec.bytecode.object.length
);

let deployWhaleBadgeHash: `0x${string}` | undefined = undefined;
try {
  console.log("⏳ Deploying helper contracts...");
  deployWhaleBadgeHash = await ethClient.deployContract({
    abi: whaleBadgeNFTSpec.abi,
    bytecode: whaleBadgeNFTSpec.bytecode.object,
    account,
  });
  console.log("📨 Deploy tx hash:", deployWhaleBadgeHash);
} catch (err) {
  console.error("❌ Error during contract deployment:");
  console.error(err);
  throw err;
}

const whaleBadgeNFTAddress = await waitForContractDeploy({
  client: ethClient,
  hash: deployWhaleBadgeHash,
});

console.log("✅ WhaleBadgeNFT deployed at:", whaleBadgeNFTAddress);

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

console.log("📦 Deploying prover and verifier contracts...");

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [],
  verifierArgs: [whaleBadgeNFTAddress],
});

console.log("📨 Prover:", prover);
console.log("Verifier:", verifier);
console.log("teleportConfig.tokenHolder:", teleportConfig.tokenHolder);
console.log("tokensToCheck:", tokensToCheck);
console.log("chainId:", chain.id);

const proofHash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "crossChainBalanceOf",
  args: [teleportConfig.tokenHolder, tokensToCheck],
  chainId: chain.id,
});
const result = await vlayer.waitForProvingResult({ hash: proofHash });
console.log("Proof:", result[0]);
console.log("⏳ Verifying...");

const verificationHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "claim",
  args: result,
  account,
});

console.log("📨 Verification tx hash:", verificationHash);

const receipt = await ethClient.waitForTransactionReceipt({
  hash: verificationHash,
  confirmations,
  retryCount: 60,
  retryDelay: 1000,
});

console.log(`✅ Verification result: ${receipt.status}`);
