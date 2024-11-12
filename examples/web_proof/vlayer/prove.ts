import { createVlayerClient } from "@vlayer/sdk";
import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";
import tls_proof from "./tls_proof.json";
import * as assert from "assert";
import { encodePacked, isAddress, keccak256 } from "viem";
import { getConfig } from "./config";
import { waitForContractAddr, exampleContext } from "./helpers";

const config = getConfig();
const { chain, ethClient, deployer, proverUrl, confirmations } =
  await exampleContext(config);

const notaryPubKey =
  "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";
let hash = await ethClient.deployContract({
  abi: webProofProver.abi,
  bytecode: webProofProver.bytecode.object,
  account: deployer,
  args: [],
  chain,
});
console.log("Deploying Prover...");
const prover = await waitForContractAddr(ethClient, hash);
console.log("Prover deployed:", prover);

console.log("Deploying Verifier...");
hash = await ethClient.deployContract({
  abi: webProofVerifier.abi,
  bytecode: webProofVerifier.bytecode.object,
  account: deployer,
  args: [prover],
  chain,
});
const verifier = await waitForContractAddr(ethClient, hash);
console.log("Verifier deployed:", verifier);

const twitterUserAddress = deployer.address;

const vlayer = createVlayerClient({
  url: proverUrl,
});

await testSuccessProvingAndVerification();
await testFailedProving();

async function testSuccessProvingAndVerification() {
  console.log("Proving...");

  const webProof = { tls_proof: tls_proof, notary_pub_key: notaryPubKey };

  const hash = await vlayer.prove({
    address: prover,
    functionName: "main",
    proverAbi: webProofProver.abi,
    args: [
      {
        webProofJson: JSON.stringify(webProof),
      },
      twitterUserAddress,
    ],
    chainId: chain.id,
  });
  const result = await vlayer.waitForProvingResult(hash);
  const [proof, twitterHandle, address] = result;
  console.log("Proof:", proof);

  if (!isAddress(address)) {
    throw new Error(`${address} is not a valid address`);
  }

  console.log("Verifying...");

  const txHash = await ethClient.writeContract({
    address: verifier,
    abi: webProofVerifier.abi,
    functionName: "verify",
    args: [proof, twitterHandle, address],
    chain,
    account: deployer,
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
    abi: webProofVerifier.abi,
    functionName: "balanceOf",
    args: [twitterUserAddress],
  });

  assert.strictEqual(balance, 1n);

  const tokenOwnerAddress = await ethClient.readContract({
    address: verifier,
    abi: webProofVerifier.abi,
    functionName: "ownerOf",
    args: [generateTokenId(twitterHandle)],
  });

  assert.strictEqual(twitterUserAddress, tokenOwnerAddress);
}

async function testFailedProving() {
  console.log("Proving...");

  const wrongWebProof = { tls_proof: tls_proof, notary_pub_key: "wrong" };

  try {
    const hash = await vlayer.prove({
      address: prover,
      functionName: "main",
      proverAbi: webProofProver.abi,
      args: [
        {
          webProofJson: JSON.stringify(wrongWebProof),
        },
        twitterUserAddress,
      ],
      chainId: chain.id,
    });
    await vlayer.waitForProvingResult(hash);
    throw new Error("Proving should have failed!");
  } catch (error) {
    assert.ok(error instanceof Error, `Invalid error returned: ${error}`);
    assert.equal(
      error.message,
      "Error response: Host error: TravelCallExecutor error: EVM transact error: ASN.1 error: PEM error: PEM preamble contains invalid data (NUL byte) at line 1 column 22883",
      `Error with wrong message returned: ${error.message}`,
    );
    console.log("Proving failed as expected with message:", error.message);
  }
}

function generateTokenId(username: string): bigint {
  return BigInt(keccak256(encodePacked(["string"], [username])));
}
