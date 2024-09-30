import { prove, testHelpers } from "@vlayer/sdk";
import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";
import tls_proof from "./tls_gp_proof.json";
import * as assert from "assert";
import { encodePacked, keccak256, type Address } from "viem";

const notaryPubKey =
  "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n";
const webProof = { tls_proof: tls_proof, notary_pub_key: notaryPubKey };
const wrongWebProof = { tls_proof: tls_proof, notary_pub_key: "wrong" };

const [prover, verifier] = await testHelpers.deployProverVerifier(
  webProofProver,
  webProofVerifier,
);
testSuccessProvingAndVerification(prover, verifier);
testFailedProving(prover);

async function testSuccessProvingAndVerification(
  prover: Address,
  verifier: Address,
) {
  console.log("Proving...");
  const { proof, returnValue } = await prove(
    prover,
    webProofProver.abi,
    "main",
    [
      {
        webProofJson: JSON.stringify(webProof),
      },
    ],
  );
  console.log("Proof:", proof);

  const twitterUserAddress = (await testHelpers.getTestAddresses())[0];

  console.log("Verifying...");
  await testHelpers.writeContract(
    verifier,
    webProofVerifier.abi,
    "verify",
    [proof, returnValue],
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
    [generateTokenId("g_p_vlayer")],
  );

  assert.strictEqual(twitterUserAddress, tokenOwnerAddress);
}

async function testFailedProving(prover: Address) {
  console.log("Proving...");
  try {
    await prove(prover, webProofProver.abi, "main", [
      {
        webProofJson: JSON.stringify(wrongWebProof),
      },
    ]);
  } catch (error) {
    if (
      error instanceof Error &&
      error.message ==
        "Error response: Host error: Engine error: EVM transact error: ASN.1 error: PEM error: PEM preamble contains invalid data (NUL byte) at line 1 column 22883"
    ) {
      console.log("Proving failed as expected with message:", error.message);
    } else if (error instanceof Error) {
      throw new Error(`Error with wrong message returned: ${error.message}`);
    } else {
      throw new Error(`Invalid error returned: ${error}`);
    }
    return;
  }

  throw new Error("Proving should have failed!");
}

function generateTokenId(username: string): bigint {
  return BigInt(keccak256(encodePacked(["string"], [username])));
}
