import lotrApiProverSpec from "../../contracts/fixtures/out/LotrApiProver.sol/LotrApiProver";
import emailProverSpec from "../../contracts/fixtures/out/EmailProver.sol/EmailProver";

import { writeEnvVariables, deployProver, getConfig } from "@vlayer/sdk/config";

const config = getConfig();

const unconditionalProver = await deployProver({
  proverSpec: lotrApiProverSpec,
});

const emailProver = await deployProver({
  proverSpec: emailProverSpec,
});

writeEnvVariables(".env", {
  VITE_LOTR_API_PROVER_ADDRESS: unconditionalProver,
  VITE_EMAIL_PROVER_ADDRESS: emailProver,
  VITE_VLAYER_API_TOKEN: config.token,
});
