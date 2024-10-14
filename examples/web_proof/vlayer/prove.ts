import { testHelpers, createVlayerClient } from "@vlayer/sdk";
import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";
import tls_proof from "./accountable_tls_proof.json";
import * as assert from "assert";
import { encodePacked, keccak256 } from "viem";

const notaryPubKey =
  "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEBv36FI4ZFszJa0DQFJ3wWCXvVLFr\ncRzMG5kaTeHGoSzDu6cFqx3uEWYpFGo6C0EOUgf+mEgbktLrXocv5yHzKg==\n-----END PUBLIC KEY-----\n";

const [prover, verifier] = await testHelpers.deployProverVerifier(
  webProofProver,
  webProofVerifier,
);

const twitterUserAddress = (await testHelpers.getTestAddresses())[0];

const vlayer = createVlayerClient();

await testSuccessProvingAndVerification();

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
    ],
  });
  const { proof, result } = await vlayer.waitForProvingResult({ hash });
  console.log("Proof:", proof);
  console.log("Result:", result);
}

//   console.log("Verifying...");
//   await testHelpers.writeContract(
//     verifier,
//     webProofVerifier.abi,
//     "verify",
//     [proof, ...result],
//     twitterUserAddress,
//   );
//   console.log("Verified!");

//   const balance = await testHelpers.call(
//     webProofVerifier.abi,
//     verifier,
//     "balanceOf",
//     [twitterUserAddress],
//   );

//   assert.strictEqual(balance, 1n);

//   const tokenOwnerAddress = await testHelpers.call(
//     webProofVerifier.abi,
//     verifier,
//     "ownerOf",
//     [generateTokenId(result[0])],
//   );

//   assert.strictEqual(twitterUserAddress, tokenOwnerAddress);
// }

