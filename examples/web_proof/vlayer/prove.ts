import { testHelpers, createVlayerClient } from "@vlayer/sdk";
import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";
import { serverSideTlsnProofProvider } from "./server-side-sdk-provider";
const notaryPubKey =
  "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEBv36FI4ZFszJa0DQFJ3wWCXvVLFr\ncRzMG5kaTeHGoSzDu6cFqx3uEWYpFGo6C0EOUgf+mEgbktLrXocv5yHzKg==\n-----END PUBLIC KEY-----\n";

const [prover] = await testHelpers.deployProverVerifier(
  webProofProver,
  webProofVerifier,
);

const vlayer = createVlayerClient();

async function testSuccessProvingAndVerification() {
  console.log("Proving...");

  const tls_proof = await serverSideTlsnProofProvider.getWebProof(
    "https://www.accountable.capital:10443/binance",
  );

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

  console.log("Proved!");
  console.log("Verifying...", proof, result);
  return { proof, result };
}

testSuccessProvingAndVerification();

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
