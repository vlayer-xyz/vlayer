import { helpers, getContractSpec, prove } from "vlayer-sdk";
import * as path from "path";
import { type Address } from "viem";

const PROVER = "SimpleProver";
const FILE = path.join(__dirname, `../out/${PROVER}.sol/${PROVER}.json`)
const PROVER_SPEC = await getContractSpec(FILE);
const FUNCTION_NAME = 'sum'
const ARGS = [1, 2]

console.log("Deploying prover")
let prover: Address = await helpers.deployContract(PROVER_SPEC);
console.log(`Prover has been deployed on ${prover} address`);

let blockNo = Number(await helpers.client().getBlockNumber());
console.log(`Running proving on ${blockNo} block number`);

console.log("Proving...");
let response = await prove(prover, PROVER_SPEC, FUNCTION_NAME, ARGS, blockNo);
console.log("Response:")
console.log(response);
