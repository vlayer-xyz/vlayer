import fs from "fs";
import { createVlayerClient, preverifyEmail, type ProveArgs } from "@vlayer/sdk";
import proverSpec from "../out/EmailDomainProver.sol/EmailDomainProver";
import verifierSpec from "../out/EmailProofVerifier.sol/EmailDomainVerifier";
import {
  createContext,
  deployVlayerContracts,
  getConfig,
} from "@vlayer/sdk/config";

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

console.log("Proving...");
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
  gasLimit: config.gasLimit,
  args: [emailArgs],
} as ProveArgs<typeof proverSpec.abi, "main">;
const { proverAbi, ...argsToLog } = proveArgs;
console.log("Proving args:", argsToLog);

const hash = await vlayer.prove(proveArgs);
console.log("Proving hash:", hash);

const result = await vlayer.waitForProvingResult({ hash });
console.log("Proving result:", result);

console.log("Verifying...");

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

console.log(`Verification result: ${receipt.status}`);
