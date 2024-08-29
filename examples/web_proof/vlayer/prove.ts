import {testHelpers, prove} from "@vlayer/sdk";
import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";
import tls_proof from './tls_proof.json';

const notaryPubKey = "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n"
const webProof = {tls_proof: tls_proof, notary_pub_key: notaryPubKey}

const [prover, verifier] = await testHelpers.deployProverVerifier(webProofProver, webProofVerifier);

console.log("Proving...");
const {proof, returnValue} = await prove(prover, webProofProver.abi, 'main', [{
  webProofJson: JSON.stringify(webProof)
}]);
console.log("Proof:", proof)

console.log("Verifying...")
await testHelpers.writeContract(verifier, webProofVerifier.abi, 'verify', [proof, returnValue]);
console.log('Verified!');