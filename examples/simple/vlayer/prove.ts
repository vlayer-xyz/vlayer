import { createVlayerClient, type ProveArgs } from "@vlayer/sdk";
import nftSpec from "../out/ExampleNFT.sol/ExampleNFT";
import tokenSpec from "../out/ExampleToken.sol/ExampleToken";
import {
  getConfig,
  createContext,
  deployVlayerContracts,
  waitForContractDeploy,
} from "@vlayer/sdk/config";
import debug from "debug";

import proverSpec from "../out/SimpleProver.sol/SimpleProver";
import verifierSpec from "../out/SimpleVerifier.sol/SimpleVerifier";

const createLogger = (namespace: string) => {
  const debugLogger = debug(namespace + ":debug");
  const infoLogger = debug(namespace + ":info");

  // Enable info logs by default
  if (!debug.enabled(namespace + ":info")) {
    debug.enable(`${debug.disable()},${namespace}:info`);
  }

  return {
    info: (message: string, ...args: unknown[]) => infoLogger(message, ...args),
    debug: (message: string, ...args: unknown[]) =>
      debugLogger(message, ...args),
  };
};

const log = createLogger("examples:simple");

const config = getConfig();
const {
  chain,
  ethClient,
  account: john,
  proverUrl,
  confirmations,
} = createContext(config);

if (!john) {
  throw new Error(
    "No account found make sure EXAMPLES_TEST_PRIVATE_KEY is set in your environment variables",
  );
}

const INITIAL_TOKEN_SUPPLY = BigInt(10_000_000);

const tokenDeployTransactionHash = await ethClient.deployContract({
  abi: tokenSpec.abi,
  bytecode: tokenSpec.bytecode.object,
  account: john,
  args: [john.address, INITIAL_TOKEN_SUPPLY],
});

const tokenAddress = await waitForContractDeploy({
  client: ethClient,
  hash: tokenDeployTransactionHash,
});

const nftDeployTransactionHash = await ethClient.deployContract({
  abi: nftSpec.abi,
  bytecode: nftSpec.bytecode.object,
  account: john,
  args: [],
});

const nftContractAddress = await waitForContractDeploy({
  client: ethClient,
  hash: nftDeployTransactionHash,
});

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [tokenAddress],
  verifierArgs: [nftContractAddress],
});

log.info("Proving...");
const vlayer = createVlayerClient({
  url: proverUrl,
  token: config.token,
});

const proveArgs = {
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "balance",
  args: [john.address],
  chainId: chain.id,
  vgasLimit: config.vgasLimit,
} as ProveArgs<typeof proverSpec.abi, "balance">;

const { proverAbi: _, ...argsToLog } = proveArgs;
log.debug("Proving args:", argsToLog);

const hash = await vlayer.prove(proveArgs);
log.debug("Proving hash:", hash);

const result = await vlayer.waitForProvingResult({ hash });
const [proof, owner, balance] = result;

log.debug("Proof result:", result);
// Workaround for viem estimating gas with `latest` block causing future block assumptions to fail on slower chains like mainnet/sepolia
const gas = await ethClient.estimateContractGas({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "claimWhale",
  args: [proof, owner, balance],
  account: john,
  blockTag: "pending",
});

const verificationHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "claimWhale",
  args: [proof, owner, balance],
  account: john,
  gas,
});

const receipt = await ethClient.waitForTransactionReceipt({
  hash: verificationHash,
  confirmations,
  retryCount: 60,
  retryDelay: 1000,
});

log.info(`Verification result: ${receipt.status}`);
