import { type Address } from "viem";

import { testHelpers, createVlayerClient } from "@vlayer/sdk";
import vToyken from "../out/VToyken.sol/VToyken";
import erc20Prover from "../out/ERC20Prover.sol/ERC20Prover";

const tokenOwners: Address[] = ["0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC"];

console.log("Deploying prover");
const token: Address = await testHelpers.deployContract(vToyken);
const prover: Address = await testHelpers.deployContract(erc20Prover, [token]);
console.log(`Prover has been deployed on ${prover} address`);

console.log("Proving...");
const vlayer = createVlayerClient();
const { hash } = vlayer.prove({
  address: prover,
  proverAbi: erc20Prover.abi,
  functionName: "prove",
  args: [tokenOwners],
});
const response = await vlayer.waitForProvingResult({ hash });
console.log("Response:");
console.log(response);
