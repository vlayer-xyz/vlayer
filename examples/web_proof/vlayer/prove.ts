import { testHelpers, createVlayerClient } from "@vlayer/sdk";
import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";
import tls_proof from "./tls_gp_proof.json";
import * as assert from "assert";
import { encodePacked, isAddress, keccak256 } from "viem";
import { foundry } from "viem/chains";

const notaryPubKey =
  "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";

const [prover, verifier] = await testHelpers.deployProverVerifier(
  webProofProver,
  webProofVerifier,
);

const twitterUserAddress = (await testHelpers.getTestAddresses())[0];

const vlayer = createVlayerClient();

await testSuccessProvingAndVerification();
await testFailedProving();

async function testSuccessProvingAndVerification() {
  console.log("Proving...");

  const webProof = { tls_proof: tls_proof, notary_pub_key: notaryPubKey };

  const { hash } = await vlayer.prove({
    address: prover,
    functionName: "main",
    proverAbi: webProofProver.abi,
    args: [
      {
        webProofJson: JSON.stringify(webProof),
      },
      twitterUserAddress,
    ],
    commitmentArgs: [twitterUserAddress],
    chainId: foundry.id,
  });
  const result = await vlayer.waitForProvingResult({ hash });
  const [proof, twitterHandle, address] = result;
  console.log("Proof:", proof);

  if (typeof twitterHandle !== "string") {
    throw new Error("Twitter handle is not a string");
  }

  if (typeof address !== "string" || !isAddress(address)) {
    throw new Error(`${address} is not a valid address`);
  }

  console.log("Verifying...");
  await testHelpers.writeContract(
    verifier,
    webProofVerifier.abi,
    "verify",
    [proof, twitterHandle, address],
    twitterUserAddress,
  );
  console.log("Verified!");

  const balance = await testHelpers.call(
    webProofVerifier.abi,
    verifier,
    "balanceOf",
    [twitterUserAddress],
  );

  assert.strictEqual(balance, 1n);

  const tokenOwnerAddress = await testHelpers.call(
    webProofVerifier.abi,
    verifier,
    "ownerOf",
    [generateTokenId(twitterHandle)],
  );

  assert.strictEqual(twitterUserAddress, tokenOwnerAddress);
}

async function testFailedProving() {
  console.log("Proving...");

  const wrongWebProof = { tls_proof: tls_proof, notary_pub_key: "wrong" };

  try {
    const { hash } = await vlayer.prove({
      address: prover,
      functionName: "main",
      proverAbi: webProofProver.abi,
      args: [
        {
          webProofJson: JSON.stringify(wrongWebProof),
        },
        twitterUserAddress,
      ],
      commitmentArgs: [twitterUserAddress],
      chainId: foundry.id,
    });
    await vlayer.waitForProvingResult({ hash });
    throw new Error("Proving should have failed!");
  } catch (error) {
    assert.ok(error instanceof Error, `Invalid error returned: ${error}`);
    assert.equal(
      error.message,
      "Error response: Host error: Engine error: EVM transact error: ASN.1 error: PEM error: PEM preamble contains invalid data (NUL byte) at line 1 column 22883",
      `Error with wrong message returned: ${error.message}`,
    );
    console.log("Proving failed as expected with message:", error.message);
  }
}

function generateTokenId(username: string): bigint {
  return BigInt(keccak256(encodePacked(["string"], [username])));
}
