import { type Address } from "viem";

import { deployContract } from "../api/helpers";
import fakeProofVerifier from "@contracts/FakeProofVerifier.sol/FakeProofVerifier.json";
import { ContractSpec } from "../api/prover";

//TODO : check why this cast is needed and type is not properly inferred

const addr: Address = await deployContract(fakeProofVerifier as ContractSpec);
console.log(`fakeProofVerifier addr: ${addr}`);
