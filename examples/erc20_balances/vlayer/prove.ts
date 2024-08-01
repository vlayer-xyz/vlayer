import { helpers, getContractSpec, prove } from "vlayer-sdk";
import * as path from "path";
import { type Address } from "viem";

const TOKEN = "VToyken";
const TOKEN_FILE = path.join(__dirname, `../out/${TOKEN}.sol/${TOKEN}.json`)
const TOKEN_SPEC = await getContractSpec(TOKEN_FILE);

const PROVER = "ERC20Prover";
const FILE = path.join(__dirname, `../out/${PROVER}.sol/${PROVER}.json`)
const PROVER_SPEC = await getContractSpec(FILE);
const FUNCTION_NAME = 'prove'
const TOKEN_OWNERS: Address[] =  ['0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC'];
const ARGS = [TOKEN_OWNERS];

console.log("Deploying prover")
let token: Address = await helpers.deployContract(TOKEN_SPEC);
let prover: Address = await helpers.deployContract(PROVER_SPEC, [token]);
console.log(`Prover has been deployed on ${prover} address`);

let blockNo = Number(await helpers.client().getBlockNumber());
console.log(`Running proving on ${blockNo} block number`);

console.log("Proving...");
let response = await prove(prover, PROVER_SPEC, FUNCTION_NAME, ARGS, blockNo);
console.log("Response:")
console.log(response);
