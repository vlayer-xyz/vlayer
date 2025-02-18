import lotrApiProverSpec from "../../contracts/fixtures/out/LotrApiProver.sol/LotrApiProver";
import emailProverSpec from "../../contracts/fixtures/out/EmailProver.sol/EmailProver";

import { writeEnvVariables, deployProver } from "@vlayer/sdk/config";

const unconditionalProver = await deployProver({
  proverSpec: lotrApiProverSpec,
});

const emailProver = await deployProver({
  proverSpec: emailProverSpec,
});

void writeEnvVariables(".env", {
  VITE_LOTR_API_PROVER_ADDRESS: unconditionalProver,
  VITE_EMAIL_PROVER_ADDRESS: emailProver,
});
