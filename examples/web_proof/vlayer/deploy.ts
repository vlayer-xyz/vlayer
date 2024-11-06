import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";
import { updateDotFile, getEthClient } from "./helpers";
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
let receipt = await ethClient.waitForTransactionReceipt({
  hash,
});
if (receipt.status != "success") {
  throw new Error(`Prover deployment failed with status: ${receipt.status}`);
}
const prover = receipt.contractAddress;
console.log(`Prover deployed to ${config.chainName}`, prover);

hash = await ethClient.deployContract({
  abi: webProofVerifier.abi,
  bytecode: webProofVerifier.bytecode.object,
  account: config.deployer,
  args: [prover],
  chain: config.chain,
});

receipt = await ethClient.waitForTransactionReceipt({
  hash,
});
const verifier = receipt.contractAddress;
console.log(`Verifier deployed to ${config.chainName}`, verifier);

if (receipt.status != "success") {
  throw new Error(`Verifier deployment failed with status: ${receipt.status}`);
}

if (!verifier || !prover) throw new Error("verifier or prover not deployed");

await updateDotFile(config.envPath, {
  VITE_PROVER_ADDRESS: prover,
  VITE_VERIFIER_ADDRESS: verifier,
});
