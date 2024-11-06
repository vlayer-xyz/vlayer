import path from "node:path";
import webProofProver from "../out/WebProofProver.sol/WebProofProver";
import webProofVerifier from "../out/WebProofVerifier.sol/WebProofVerifier";
import { updateDotFile, loadDotFile, getConfig } from "./helpers";

try {
  const envPath = path.resolve(__dirname, ".env.development");
  await loadDotFile(envPath);
  const config = await getConfig();

  let hash = await config.walletClient.deployContract({
    abi: webProofProver.abi,
    bytecode: webProofProver.bytecode.object,
    account: config.deployer,
    args: [],
    chain: config.chain,
  });
  let receipt = await config.publicClient.waitForTransactionReceipt({
    hash,
  });
  if (receipt.status != "success") {
    throw new Error(`Prover deployment failed with status: ${receipt.status}`);
  }
  const prover = receipt.contractAddress;
  console.log(`Prover deployed to ${config.chainName}`, prover);

  hash = await config.walletClient.deployContract({
    abi: webProofVerifier.abi,
    bytecode: webProofVerifier.bytecode.object,
    account: config.deployer,
    args: [prover],
    chain: config.chain,
  });

  receipt = await config.publicClient.waitForTransactionReceipt({
    hash,
  });
  const verifier = receipt.contractAddress;
  console.log(`Verifier deployed to ${config.chainName}`, verifier);

  if (receipt.status != "success") {
    throw new Error(
      `Verifier deployment failed with status: ${receipt.status}`,
    );
  }

  await updateDotFile(envPath, {
    VITE_PROVER_ADDRESS: prover,
    VITE_VERIFIER_ADDRESS: verifier,
  });
} catch (err) {
  console.error("Error updating the .env.development file:", err);
}
