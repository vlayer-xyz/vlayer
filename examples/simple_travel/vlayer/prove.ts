import { deployContract, getContractSpec, prove } from "vlayer-sdk";
import * as path from "path";
import { type Address } from "viem";
import { client } from "../../../packages/src/api/helpers";

const COUNTER = "Counter";
const COUNTER_FILE = path.join(__dirname, `../out/${COUNTER}.sol/${COUNTER}.json`)
const COUNTER_SPEC = await getContractSpec(COUNTER_FILE);

const PROVER = "SimpleTravelProver";
const FILE = path.join(__dirname, `../out/${PROVER}.sol/${PROVER}.json`)
const PROVER_SPEC = await getContractSpec(FILE);
const FUNCTION_NAME = 'aroundTheWorld'
const ARGS: any[] = []
const CALLER_ADDRESS = "0x0000000000000000000000000000000000000000";


console.log("Deploying prover")
let counter: Address = await deployContract(COUNTER_SPEC);
let prover: Address = await deployContract(PROVER_SPEC, [counter]);
console.log(`Prover has been deployed on ${prover} address`);

let blockNo = Number(await client().getBlockNumber());
console.log(`Running proving on ${blockNo} block number`);

console.log("Proving...");
let response = await prove(CALLER_ADDRESS, prover, PROVER_SPEC, FUNCTION_NAME, ARGS, blockNo);
console.log("Response:")
console.log(response);
