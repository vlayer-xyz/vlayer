const PROVER_ADDRESS = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
import unconditionalProver from "../../../../contracts/fixtures/out/UnconditionalProver.sol/UnconditionalProver";
import { expectUrl, notarize, startPage } from "@vlayer/sdk/web_proof";
import { foundry } from "viem/chains";

export const config = {
  proverCallCommitment: {
    address: PROVER_ADDRESS as `0x${string}`,
    proverAbi: unconditionalProver.abi,
    chainId: foundry.id,
    functionName: "web_proof",
    commitmentArgs: [] as [],
  },
  logoUrl: "",
  steps: [
    startPage("https://demo.tink.com/", "Go to tink"),
    expectUrl("https://demo.tink.com/account-check", "Go to account check"),
    notarize("https://demo.tink.com/api/report?*", "GET", "Prove"),
  ],
};
