import fs from "fs";
import {
  createVlayerClient,
  preverifyEmail,
  type ProveArgs,
} from "@vlayer/sdk";
import proverSpec from "../out/EmailDomainProver.sol/EmailDomainProver";
import verifierSpec from "../out/EmailProofVerifier.sol/EmailDomainVerifier";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
} from "@vlayer/sdk/config";
import debug from "debug";

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

const log = createLogger("examples:simple-email-proof");

const mimeEmail = fs.readFileSync("../testdata/verify_vlayer.eml").toString();

const config = getConfig();

const {
  chain,
  ethClient,
  account: john,
  proverUrl,
  dnsServiceUrl,
  confirmations,
} = createContext(config);

if (!john) {
  throw new Error(
    "No account found make sure EXAMPLES_TEST_PRIVATE_KEY is set in your environment variables",
  );
}

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: [],
  verifierArgs: [],
});

if (!dnsServiceUrl) {
  throw new Error("DNS service URL is not set");
}

log.info("Proving...");
const vlayer = createVlayerClient({
  url: proverUrl,
  token: config.token,
});
const emailArgs = await preverifyEmail({
  mimeEmail,
  dnsResolverUrl: dnsServiceUrl,
  token: config.token,
});

const proveArgs = {
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "main",
  chainId: chain.id,
  vgasLimit: config.vgasLimit,
  args: [emailArgs],
} as ProveArgs<typeof proverSpec.abi, "main">;
const { proverAbi: _, ...argsToLog } = proveArgs;
log.debug("Proving args:", argsToLog);

const hash = await vlayer.prove(proveArgs);
log.debug("Proving hash:", hash);

const result = await vlayer.waitForProvingResult({ hash });
log.debug("Proving result:", result);

log.info("Verifying...");

// Workaround for viem estimating gas with `latest` block causing future block assumptions to fail on slower chains like mainnet/sepolia
const gas = await ethClient.estimateContractGas({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "verify",
  args: result,
  account: john,
  blockTag: "pending",
});

const verificationHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "verify",
  args: result,
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
