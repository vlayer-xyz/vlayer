import {
  testHelpers,
  createVlayerClient,
  createExtensionWebProofProvider,
} from "@vlayer/sdk";
import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";
import tls_proof from "./tls_gp_proof.json";
import * as assert from "assert";
import { encodePacked, keccak256 } from "viem";

const notaryPubKey =
  "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";

const [prover, verifier] = await testHelpers.deployProverVerifier(
  webProofProver,
  webProofVerifier,
);

const twitterUserAddress = (await testHelpers.getTestAddresses())[0];

testSuccessProvingAndVerification();
testFailedProving();

async function testSuccessProvingAndVerification() {
  console.log("Proving...");

  const webProof = { tls_proof: tls_proof, notary_pub_key: notaryPubKey };
  const client = createVlayerClient({
    url: "x.com",
    webProofProvider: createExtensionWebProofProvider({}),
  });

  const { proof, returnValue } = await client.prove({
    address: prover,
    functionName: "main",
    proverAbi: webProofProver.abi,
    args: [
      {
        webProofJson: JSON.stringify(webProof),
      },
      twitterUserAddress,
    ],
  });
  console.log("Proof:", proof);

  console.log("Verifying...");
  await testHelpers.writeContract(
    verifier,
    webProofVerifier.abi,
    "verify",
    [proof, ...returnValue],
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
    [generateTokenId(returnValue[0])],
  );

  assert.strictEqual(twitterUserAddress, tokenOwnerAddress);
}

async function testFailedProving() {
  console.log("Proving...");

  const wrongWebProof = { tls_proof: tls_proof, notary_pub_key: "wrong" };
  const client = createVlayerClient({
    url: "x.com",
    webProofProvider: wrongWebProof,
  });

  try {
    await client.prove({
      address: prover,
      functionName: "main",
      proverAbi: webProofProver.abi,
      args: [
        {
          webProofJson: JSON.stringify(wrongWebProof),
        },
        twitterUserAddress,
      ],
    });
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
