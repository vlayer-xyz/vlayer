import proverSpec from "../out/WebProofProver.sol/WebProofProver";
import verifierSpec from "../out/WebProofVerifier.sol/WebProofVerifier";
import {
  deployVlayerContracts,
  writeEnvVariables,
  getConfig,
} from "@vlayer/sdk/config";

const config = getConfig();

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  env: config.deployConfig,
});

writeEnvVariables(".env", {
  VITE_PROVER_ADDRESS: prover,
  VITE_VERIFIER_ADDRESS: verifier,
  VITE_CHAIN_NAME: config.chainName,
  VITE_PROVER_URL: config.proverUrl,
  VITE_JSON_RPC_URL: config.jsonRpcUrl,
  VITE_PRIVATE_KEY: config.privateKey,
  VITE_VLAYER_API_TOKEN: config.token,
  VITE_USE_WINDOW_ETHEREUM_TRANSPORT:
    process.env.USE_WINDOW_ETHEREUM_TRANSPORT || "",
});
