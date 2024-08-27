import { type Address } from "viem";

import { deployContract } from "../api/helpers";
import fakeProofVerifier from "@contracts/FakeProofVerifier.sol/FakeProofVerifier.json";

const addr: Address = await deployContract(fakeProofVerifier);
console.log(`fakeProofVerifier addr: ${addr}`);
