import { createVlayerClient, type ProveArgs } from "@vlayer/sdk";
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
import debug from "debug";

const createLogger = (namespace: string) => {
  const debugLogger = debug(namespace + ":debug");
  const infoLogger = debug(namespace + ":info");

  // Enable info logs by default
  if (!debug.enabled(namespace + ":info")) {
    debug.enable(namespace + ":info");
  }

  return {
    info: (message: string, ...args: unknown[]) => infoLogger(message, ...args),
    debug: (message: string, ...args: unknown[]) =>
      debugLogger(message, ...args),
  };
};

const log = createLogger("examples:simple-teleport");

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
log.info("⏳ Deploying helper contracts...");
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

const proveArgs = {
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "crossChainBalanceOf",
  args: [teleportConfig.tokenHolder, tokensToCheck],
  chainId: chain.id,
  gasLimit: config.gasLimit,
} as ProveArgs<typeof proverSpec.abi, "crossChainBalanceOf">;
const { proverAbi: _, ...argsToLog } = proveArgs;
log.debug("Proving args:", argsToLog);

const proofHash = await vlayer.prove(proveArgs);
log.debug("Proving hash:", proofHash);

const result = await vlayer.waitForProvingResult({ hash: proofHash });
log.debug("Proving result:", result);

log.info("⏳ Verifying...");

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
  retryCount: 600,
  retryDelay: 1000,
  timeout: 600 * 1000,
});

log.info(`✅ Verification result: ${receipt.status}`);
