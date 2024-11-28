// import proverSpec from "../out/EmailDomainProver.sol/EmailDomainProver";
// import verifierSpec from "../out/EmailProofVerifier.sol/EmailDomainVerifier";

import {
  //   deployVlayerContracts,
  writeEnvVariables,
  getConfig,
} from "@vlayer/sdk/config";

// const { prover, verifier } = await deployVlayerContracts({
//   proverSpec,
//   verifierSpec,
//   proverArgs: ["@vlayer.xyz"],
//   verifierArgs: ["vlayer badge", "VL"],
// });

const config = getConfig();

writeEnvVariables(".env", {
  //   VITE_PROVER_ADDRESS: prover,
  //   VITE_VERIFIER_ADDRESS: verifier,
  VITE_CHAIN_NAME: config.chainName,
  VITE_PROVER_URL: config.proverUrl,
  VITE_JSON_RPC_URL: config.jsonRpcUrl,
  VITE_PRIVATE_KEY: config.privateKey,
  VITE_USE_WINDOW_ETHEREUM_TRANSPORT:
    process.env.USE_WINDOW_ETHEREUM_TRANSPORT || "",
  VITE_NOTARY_URL: process.env.NOTARY_URL || "",
  VITE_WS_PROXY_URL: process.env.WS_PROXY_URL || "",
});
