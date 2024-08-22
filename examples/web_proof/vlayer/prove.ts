import type { Address } from "viem";

import { testHelpers, prove } from "@vlayer/sdk";
import webProofProver from "../out/WebProofProver.sol/WebProofProver.json";
import tls_proof from './tls_proof.json';

const NOTARY_PUB_KEY_PEM = "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n"
const WEB_PROOF = {tls_proof: tls_proof, notary_pub_key: NOTARY_PUB_KEY_PEM}
const ARGS: any[] = [[JSON.stringify(WEB_PROOF)]];

console.log("Deploying prover")
let prover: Address = await testHelpers.deployContract(webProofProver);
console.log(`Prover has been deployed on ${prover} address`);

console.log("Proving...");
let response = await prove(prover, webProofProver, 'main', ARGS);
console.log("Response:", response)
