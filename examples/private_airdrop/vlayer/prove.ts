import { helpers, getContractSpec, prove } from "vlayer-sdk";
import * as path from "path";
import { type Address } from "viem";
import { privateKeyToAccount, generatePrivateKey } from 'viem/accounts'

const TOKEN = "VToyken";
const TOKEN_FILE = path.join(__dirname, `../out/${TOKEN}.sol/${TOKEN}.json`)
const TOKEN_SPEC = await getContractSpec(TOKEN_FILE);

const PROVER = "PrivateAirdropProver";
const FILE = path.join(__dirname, `../out/${PROVER}.sol/${PROVER}.json`)
const PROVER_SPEC = await getContractSpec(FILE);
const FUNCTION_NAME = 'prove'

const client = helpers.client();
const account = privateKeyToAccount("0xd913a2f110382ecb4046247e20d59147cb04bc6e1205995c6332bc4cad643a6d")
const signature = await client.signMessage({ 
  account,
  message: 'erc20 prover',
})

const ARGS = [account.address, signature];

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
