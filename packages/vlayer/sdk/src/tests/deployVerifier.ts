import { deployContract } from "../api/helpers";
import { getContractSpec } from "../api/prover";
import * as path from "path";
import { type Address } from "viem";

const CONTRACT = path.join(__dirname, `../../../contracts/out/FakeProofVerifier.sol/FakeProofVerifier.json`);
const CONTRACT_SPEC = await getContractSpec(CONTRACT);

let addr: Address = await deployContract(CONTRACT_SPEC);
console.log(addr);
