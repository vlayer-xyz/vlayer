import { deployProver, getProverSpec, prove } from "vlayer-sdk";
import * as path from "path";
import { type Address } from "viem";
import { client } from "../../../packages/src/api/helpers";

const PROVER = "SimpleProver";
const FILE = path.join(__dirname, `../out/${PROVER}.sol/${PROVER}.json`)
const PROVER_SPEC = await getProverSpec(FILE);
const FUNCTION_NAME = 'sum'
const ARGS = [1, 2]
const CALLER_ADDRESS = "0x0000000000000000000000000000000000000000";


let prover: Address = await deployProver(PROVER_SPEC);

let blockNo = Number(await client().getBlockNumber());

let response = await prove(CALLER_ADDRESS, prover, PROVER_SPEC, FUNCTION_NAME, ARGS, blockNo);

console.log(response);
