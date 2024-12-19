import { createVlayerClient } from "@vlayer/sdk";
import proverSpec from "../out/WebProofProver.sol/WebProofProver";
import verifierSpec from "../out/WebProofVerifier.sol/WebProofVerifier";
import web_proof from "./web_proof.json";
import web_proof_invalid_signature from "./web_proof_invalid_signature.json";
import * as assert from "assert";
import { encodePacked, isAddress, keccak256 } from "viem";

import {
  getConfig,
  createContext,
  deployVlayerContracts,
  writeEnvVariables,
} from "@vlayer/sdk/config";

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
});

writeEnvVariables(".env", {
  VITE_PROVER_ADDRESS: prover,
  VITE_VERIFIER_ADDRESS: verifier,
});

const config = getConfig();
const { chain, ethClient, account, proverUrl, confirmations } =
  await createContext(config);

const twitterUserAddress = account.address;
const vlayer = createVlayerClient({
  url: proverUrl,
});

await testSuccessProvingAndVerification();
await testFailedProving();

async function testSuccessProvingAndVerification() {
  console.log("Proving...");

  const hash = await vlayer.prove({
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
    token: config.token,
  });
  const result = await vlayer.waitForProvingResult({ hash });
  const [proof, twitterHandle, address] = result;
  console.log("Has Proof");

  if (!isAddress(address)) {
    throw new Error(`${address} is not a valid address`);
  }

  console.log("Verifying...");

  const txHash = await ethClient.writeContract({
    address: verifier,
    abi: verifierSpec.abi,
    functionName: "verify",
    args: [proof, twitterHandle, address],
    chain,
    account: account,
  });

  await ethClient.waitForTransactionReceipt({
    hash: txHash,
    confirmations,
    retryCount: 60,
    retryDelay: 1000,
  });

  console.log("Verified!");

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
}

async function testFailedProving() {
  console.log("Proving...");

  try {
    const hash = await vlayer.prove({
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
      token: config.token,
    });
    await vlayer.waitForProvingResult({ hash });
    throw new Error("Proving should have failed!");
  } catch (error) {
    assert.ok(error instanceof Error, `Invalid error returned: ${error}`);
    assert.equal(
      error.message,
      "Error response: Host error: TravelCallExecutor error: EVM transact error: Verification error: Presentation error: presentation error: attestation error caused by: attestation proof error: signature error caused by: signature verification failed: invalid secp256k1 signature",
      `Error with wrong message returned: ${error.message}`,
    );
    console.log("Proving failed as expected with message:", error.message);
  }
}

function generateTokenId(username: string): bigint {
  return BigInt(keccak256(encodePacked(["string"], [username])));
}
