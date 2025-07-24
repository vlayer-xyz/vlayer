import { createVlayerClient, type ProveArgs } from "@vlayer/sdk";
import proverSpec from "../out/WebProofProver.sol/WebProofProver";
import verifierSpec from "../out/WebProofVerifier.sol/WebProofVerifier";
import web_proof_development_signature from "../testdata/web_proof_development_signature.json";
import web_proof_vlayer_signature from "../testdata/web_proof_vlayer_signature.json";
import web_proof_invalid_signature from "../testdata/web_proof_invalid_signature.json";
import * as assert from "assert";
import { encodePacked, keccak256 } from "viem";
import debug from "debug";

import {
  getConfig,
  createContext,
  deployVlayerContracts,
  writeEnvVariables,
} from "@vlayer/sdk/config";

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

const log = createLogger("examples:simple-web-proof");

let config = getConfig();
const web_proof =
  config.vlayerEnv === "mainnet"
    ? web_proof_vlayer_signature
    : web_proof_development_signature;

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
});

await writeEnvVariables(".env", {
  VITE_PROVER_ADDRESS: prover,
  VITE_VERIFIER_ADDRESS: verifier,
});

config = getConfig();
const { chain, ethClient, account, proverUrl, confirmations } =
  createContext(config);

if (!account) {
  throw new Error(
    "No account found make sure EXAMPLES_TEST_PRIVATE_KEY is set in your environment variables",
  );
}

const twitterUserAddress = account.address;
const vlayer = createVlayerClient({
  url: proverUrl,
  token: config.token,
});

await testSuccessProvingAndVerification({
  chain,
  ethClient,
  account,
  confirmations,
});

await testFailedProving({ chain });

async function testSuccessProvingAndVerification({
  chain,
  ethClient,
  account,
  confirmations,
}: Required<
  Pick<
    ReturnType<typeof createContext>,
    "chain" | "ethClient" | "account" | "confirmations"
  >
>) {
  log.info("Proving...");

  const proveArgs = {
    address: prover,
    functionName: "main",
    proverAbi: proverSpec.abi,
    args: [
      {
        webProofJson: JSON.stringify(web_proof),
      },
      twitterUserAddress,
    ],
    chainId: chain.id,
    gasLimit: config.gasLimit,
  } as ProveArgs<typeof proverSpec.abi, "main">;
  const { proverAbi: _, ...argsToLog } = proveArgs;
  log.debug("Proving args:", argsToLog);

  const hash = await vlayer.prove(proveArgs);
  log.debug("Proving hash:", hash);

  const result = await vlayer.waitForProvingResult({ hash });
  log.debug("Proving result:", result);

  const [proof, twitterHandle, address] = result;

  log.info("Verifying...");

  // Workaround for viem estimating gas with `latest` block causing future block assumptions to fail on slower chains like mainnet/sepolia
  const gas = await ethClient.estimateContractGas({
    address: verifier,
    abi: verifierSpec.abi,
    functionName: "verify",
    args: [proof, twitterHandle, address],
    account,
    blockTag: "pending",
  });

  const txHash = await ethClient.writeContract({
    address: verifier,
    abi: verifierSpec.abi,
    functionName: "verify",
    args: [proof, twitterHandle, address],
    chain,
    account,
    gas,
  });

  await ethClient.waitForTransactionReceipt({
    hash: txHash,
    confirmations,
    retryCount: 60,
    retryDelay: 1000,
  });

  log.info("Verified!");

  const balance = await ethClient.readContract({
    address: verifier,
    abi: verifierSpec.abi,
    functionName: "balanceOf",
    args: [twitterUserAddress],
  });

  assert.strictEqual(balance, 1n);

  const tokenOwnerAddress = await ethClient.readContract({
    address: verifier,
    abi: verifierSpec.abi,
    functionName: "ownerOf",
    args: [generateTokenId(twitterHandle)],
  });

  assert.strictEqual(twitterUserAddress, tokenOwnerAddress);

  const tokenURI = await ethClient.readContract({
    address: verifier,
    abi: verifierSpec.abi,
    functionName: "tokenURI",
    args: [generateTokenId(twitterHandle)],
  });

  assert.strictEqual(
    tokenURI,
    `https://faucet.vlayer.xyz/api/xBadgeMeta?handle=${twitterHandle}`,
  );
}

async function testFailedProving({
  chain,
}: Pick<ReturnType<typeof createContext>, "chain">) {
  try {
    const proveArgs = {
      address: prover,
      functionName: "main",
      proverAbi: proverSpec.abi,
      args: [
        {
          webProofJson: JSON.stringify(web_proof_invalid_signature),
        },
        twitterUserAddress,
      ],
      chainId: chain.id,
      gasLimit: config.gasLimit,
    } as ProveArgs<typeof proverSpec.abi, "main">;
    const { proverAbi: _, ...argsToLog } = proveArgs;
    log.debug("Proving args:", argsToLog);

    const hash = await vlayer.prove(proveArgs);
    log.debug("Proving hash:", hash);

    await vlayer.waitForProvingResult({ hash });
    throw new Error("Proving should have failed!");
  } catch (error) {
    assert.ok(
      error instanceof Error,
      `Invalid error returned: ${error as string}`,
    );
    assert.equal(
      error.message,
      'Preflight failed with error: Preflight: Transaction reverted: ContractError(Revert(Revert("Invalid notary public key")))',
      `Error with wrong message returned: ${error.message}`,
    );
    log.info("✅ Done");
  }
}

function generateTokenId(username: string): bigint {
  return BigInt(keccak256(encodePacked(["string"], [username])));
}
