import { helpers, getContractSpec, prove } from "vlayer-sdk";
import * as path from "path";
import { type Address } from "viem";
import { deployContract } from "../../../packages/src/api/helpers";
import { mainnet } from "viem/chains";

const OTHER_CHAIN_CONTRACT = "OtherChainContract"
const OTHER_CHAIN_FILE = path.join(__dirname, `../out/${OTHER_CHAIN_CONTRACT}.sol/${OTHER_CHAIN_CONTRACT}.json`)
const OTHER_CHAIN_SPEC = await getContractSpec(OTHER_CHAIN_FILE);

const PROVER = "SimpleTravelProver";
const FILE = path.join(__dirname, `../out/${PROVER}.sol/${PROVER}.json`)
const PROVER_SPEC = await getContractSpec(FILE);
const FUNCTION_NAME = 'aroundTheWorld'
const ARGS: any[] = []

console.log("Deploying a contract on mainnet chain");
let otherChainContract: Address = await deployContract(OTHER_CHAIN_SPEC, [], mainnet.id);
console.log(`Contract has been deployed on ${otherChainContract} address`);

console.log("Deploying prover on sepolia chain");
let prover: Address = await deployContract(PROVER_SPEC);
console.log(`Prover has been deployed on ${prover} address`);

let blockNo = Number(await helpers.client().getBlockNumber());
console.log(`Running proving on ${blockNo} block number`);

console.log("Proving...");
let response = await prove(prover, PROVER_SPEC, FUNCTION_NAME, ARGS, blockNo);
console.log("Response:")
console.log(response);
