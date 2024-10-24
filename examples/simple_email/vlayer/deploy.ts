import { optimismSepolia } from "viem/chains";
import {
  createPublicClient,
  createWalletClient,
  http,
  type Address,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";

import emailDomainProver from "../out/EmailDomainProver.sol/EmailDomainProver";
import emailDomainVerifier from "../out/EmailProofVerifier.sol/EmailDomainVerifier";

let privateKey = process.env.EXAMPLES_TEST_PRIVATE_KEY;

if (!privateKey) {
  throw new Error("EXAMPLES_TEST_PRIVATE_KEY environment variable is not set.");
}
privateKey = privateKey.startsWith("0x") ? privateKey : `0x${privateKey}`;
const deployer = privateKeyToAccount(privateKey as `0x${string}`);

const walletClient = createWalletClient({
  chain: optimismSepolia,
  transport: http("https://sepolia.optimism.io"),
});

const client = createPublicClient({
  chain: optimismSepolia,
  transport: http("https://sepolia.optimism.io"),
});

const deployProver = async () => {
  const txHash = await walletClient.deployContract({
    abi: emailDomainProver.abi,
    bytecode: emailDomainProver.bytecode.object,
    account: deployer,
    args: ["@vlayer.xyz"],
    chain: optimismSepolia,
  });

  const receipt = await client.waitForTransactionReceipt({ hash: txHash });

  if (receipt.status != "success") {
    throw new Error(`Prover deployment failed with status: ${receipt.status}`);
  }

  return receipt.contractAddress as Address;
};

const deployVerifier = async (prover: Address) => {
  const txHash = await walletClient.deployContract({
    abi: emailDomainVerifier.abi,
    bytecode: emailDomainVerifier.bytecode.object,
    account: deployer,
    args: [prover, "vlayer badge", "VL"],
    chain: optimismSepolia,
  });

  const receipt = await client.waitForTransactionReceipt({ hash: txHash });

  if (receipt.status != "success") {
    throw new Error(
      `Verifier deployment failed with status: ${receipt.status}`,
    );
  }

  return receipt.contractAddress as Address;
};

console.log("Deploying Prover...");
const proverAddr = await deployProver();
console.log(
  `Prover deployed: https://sepolia-optimism.etherscan.io/address/${proverAddr}`,
);

console.log("Deploying Verifier...");
const verifierAddr = await deployVerifier(proverAddr);
console.log(
  `Verifier deployed: https://sepolia-optimism.etherscan.io/address/${verifierAddr}`,
);
