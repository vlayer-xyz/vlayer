import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";
import { updateDotFile, exampleContext, waitForContractAddr } from "./helpers";
import { getConfig } from "./config";
import debug from "debug";
const log = debug("vlayer:deploy");
export const deploy = async () => {
  const config = getConfig();
  const { chain, ethClient, deployer } = await exampleContext(config);

  let hash = await ethClient.deployContract({
    abi: webProofProver.abi,
    bytecode: webProofProver.bytecode.object,
    account: deployer,
    args: [],
    chain,
  });
  const prover = await waitForContractAddr(ethClient, hash);
  log(`Prover deployed to ${config.chainName}`, prover);

  hash = await ethClient.deployContract({
    abi: webProofVerifier.abi,
    bytecode: webProofVerifier.bytecode.object,
    account: deployer,
    args: [prover],
    chain,
  });
  const verifier = await waitForContractAddr(ethClient, hash);
  log(`Verifier deployed to ${config.chainName}`, verifier);

  await updateDotFile(".env", {
    VITE_PROVER_ADDRESS: prover,
    VITE_VERIFIER_ADDRESS: verifier,
  });

  return { prover, verifier };
};
