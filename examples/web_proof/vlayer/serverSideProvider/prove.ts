import { testHelpers, createVlayerClient } from "@vlayer/sdk";

//NOTE:
// This should be replaced by actual  provider contract
import webProofProver from "../../out/WebProofProver.sol/WebProofProver";
//NOTE:
// This should be replaced by actual  verifier contract
import webProofVerifier from "../../out/WebProofVerifier.sol/WebProofVerifier";
import { serverSideTlsnProofProvider } from "./serverSideTlsProofProvider";
import { foundry } from "viem/chains";

//NOTE  this uses tlsn fixture public key as serverSideTlsProofProvider uses default lokal tlsn provider for now
//TODO: make serverSideTlsProofProvider able to accept notary server data as input

const notaryPubKey =
  "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEBv36FI4ZFszJa0DQFJ3wWCXvVLFr\ncRzMG5kaTeHGoSzDu6cFqx3uEWYpFGo6C0EOUgf+mEgbktLrXocv5yHzKg==\n-----END PUBLIC KEY-----\n";

const [prover /*,_verifier*/] = await testHelpers.deployProverVerifier(
  webProofProver,
  webProofVerifier,
);

const vlayer = createVlayerClient();

const twitterUserAddress = (await testHelpers.getTestAddresses())[0];

async function testSuccessProving() {
  console.log("Started proving...");
  //TODO : this should accept headers so private data can be accessed
  const tls_proof =
    await serverSideTlsnProofProvider.getWebProof("Your URL here");

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

  const [proof, result] = await vlayer.waitForProvingResult({ hash });

  console.log("Proof generated!", proof, result);
  return { proof, result };
}

testSuccessProving().catch((e) => {
  console.error("Error in provind", e);
});
