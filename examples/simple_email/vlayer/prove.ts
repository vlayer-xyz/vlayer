import fs from "fs";
import { createVlayerClient, preverifyEmail } from "@vlayer/sdk";
import proverSpec from "../out/EmailDomainProver.sol/EmailDomainProver";
import verifierSpec from "../out/EmailProofVerifier.sol/EmailDomainVerifier";
<<<<<<< HEAD
=======
import { foundry } from "viem/chains";
>>>>>>> c7756e26 (Use new approach to config in all examples ... (#1108))
import {
  createContext,
  deployVlayerContracts,
  getConfig,
<<<<<<< HEAD
=======
  waitForTransactionReceipt,
>>>>>>> c7756e26 (Use new approach to config in all examples ... (#1108))
} from "@vlayer/sdk/config";

const mimeEmail = fs.readFileSync("./testdata/verify_vlayer.eml").toString();

const config = getConfig();

<<<<<<< HEAD
const {
  chain,
  ethClient,
  account: john,
  proverUrl,
  confirmations,
} = await createContext(config);
=======
const { ethClient, account: john } = await createContext(config);
>>>>>>> c7756e26 (Use new approach to config in all examples ... (#1108))

const { prover, verifier } = await deployVlayerContracts({
  proverSpec,
  verifierSpec,
  proverArgs: ["@vlayer.xyz"],
  verifierArgs: ["vlayer badge", "VL"],
});

console.log("Proving...");
const vlayer = createVlayerClient({
  url: proverUrl,
});
const hash = await vlayer.prove({
  address: prover,
  proverAbi: proverSpec.abi,
  functionName: "main",
  chainId: chain.id,
  args: [await preverifyEmail(mimeEmail), john.address],
});
const result = await vlayer.waitForProvingResult(hash);
console.log("Proof:", result[0]);

console.log("Verifying...");

const verificationHash = await ethClient.writeContract({
  address: verifier,
  abi: verifierSpec.abi,
  functionName: "verify",
  args: result,
  account: john,
});

<<<<<<< HEAD
const receipt = await ethClient.waitForTransactionReceipt({
  hash: verificationHash,
  confirmations,
  retryCount: 60,
  retryDelay: 1000,
=======
const receipt = await waitForTransactionReceipt({
  hash: verificationHash,
>>>>>>> c7756e26 (Use new approach to config in all examples ... (#1108))
});

console.log(`Verification result: ${receipt.status}`);
