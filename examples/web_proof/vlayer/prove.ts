import type { Address } from "viem";

import { testHelpers, prove } from "@vlayer/sdk";
import webProofProver from "../out/WebProofProver.sol/WebProofProver.json";
import tls_proof from './tls_proof.json';

const notaryPubKey = "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n"
const webProof = { tls_proof: tls_proof, notary_pub_key: notaryPubKey }

console.log("Deploying prover")
const prover: Address = await testHelpers.deployContract(webProofProver);
console.log(`Prover has been deployed on ${prover} address`);

console.log("Proving...");
const response = await prove(prover, webProofProver, 'main', [[JSON.stringify(webProof)]]);
console.log("Response:", response)
