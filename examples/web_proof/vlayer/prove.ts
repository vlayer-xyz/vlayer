import { helpers, getContractSpec, prove } from "vlayer-sdk";
import * as path from "path";
import { type Address } from "viem";
import tls_proof from './tls_proof.json';

const PROVER = "WebProofProver";
const FILE = path.join(__dirname, `../out/${PROVER}.sol/${PROVER}.json`)
const PROVER_SPEC = await getContractSpec(FILE);
const FUNCTION_NAME = 'main'
const NOTARY_PUB_KEY_PEM = "-----BEGIN PUBLIC KEY-----\nMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAExpX/4R4z40gI6C/j9zAM39u58LJu\n3Cx5tXTuqhhu/tirnBi5GniMmspOTEsps4ANnPLpMmMSfhJ+IFHbc3qVOA==\n-----END PUBLIC KEY-----\n"
const WEB_PROOF = {tls_proof: tls_proof, notary_pub_key_pem: NOTARY_PUB_KEY_PEM}
const ARGS: any[] = [[JSON.stringify(WEB_PROOF)]];

console.log("Deploying prover")
let prover: Address = await helpers.deployContract(PROVER_SPEC);
console.log(`Prover has been deployed on ${prover} address`);

let blockNo = Number(await helpers.client().getBlockNumber());
console.log(`Running proving on ${blockNo} block number`);

console.log("Proving...");
let response = await prove(prover, PROVER_SPEC, FUNCTION_NAME, ARGS, blockNo);
console.log("Response:")
console.log(response);
