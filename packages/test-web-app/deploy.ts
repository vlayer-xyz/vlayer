import unconditionalProverSpec from "../../contracts/fixtures/out/UnconditionalProver.sol/UnconditionalProver";
import emailProverSpec from "../../contracts/fixtures/out/EmailProver.sol/EmailProver";

import {
  writeEnvVariables,
  getConfig,
  deployProver,
} from "@vlayer/sdk/config";

const unconditionalProver = await deployProver({
  proverSpec: unconditionalProverSpec,
});

const emailProver = await deployProver({
  proverSpec: emailProverSpec,
});

writeEnvVariables(".env", {
  VITE_UNCONDITIONAL_PROVER_ADDRESS: unconditionalProver,
  VITE_EMAIL_PROVER_ADDRESS: emailProver,
});
