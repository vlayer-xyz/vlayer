import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";
import { updateDotFile, getEthClient, getContractAddr } from "./helpers";
import { getConfig } from "./config";

const config = await getConfig();
const ethClient = getEthClient(config.chain, config.jsonRpcUrl);

let hash = await ethClient.deployContract({
  abi: webProofProver.abi,
  bytecode: webProofProver.bytecode.object,
  account: config.deployer,
  args: [],
  chain: config.chain,
});
const prover = await getContractAddr(ethClient, hash);
if (!prover) throw new Error("prover not deployed");
console.log(`Prover deployed to ${config.chainName}`, prover);

hash = await ethClient.deployContract({
  abi: webProofVerifier.abi,
  bytecode: webProofVerifier.bytecode.object,
  account: config.deployer,
  args: [prover],
  chain: config.chain,
});
const verifier = await getContractAddr(ethClient, hash);
if (!verifier) throw new Error("verifier not deployed");
console.log(`Verifier deployed to ${config.chainName}`, verifier);

await updateDotFile(config.envPath, {
  VITE_PROVER_ADDRESS: prover,
  VITE_VERIFIER_ADDRESS: verifier,
});
