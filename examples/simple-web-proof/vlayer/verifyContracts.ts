import { exec } from "child_process";
import { promisify } from "util";
import { getConfig, createContext } from "@vlayer/sdk/config";

const config = getConfig();
const { chain } = createContext(config);

const execAsync = promisify(exec);

async function verifyContract(
  contractAddress: string,
  contractName: string,
  constructorArguments?: string,
): Promise<void> {
  try {
    if (!process.env.CONTRACT_VERIFIER) {
      throw new Error("CONTRACT_VERIFIER is not set in environment variables");
    }
    if (!process.env.CONTRACT_VERIFIER_URL) {
      throw new Error(
        "CONTRACT_VERIFIER_URL is not set in environment variables",
      );
    }

    const command = [
      "forge verify-contract",
      contractAddress,
      contractName,
      `--chain-id ${chain.id}`,
      `--verifier ${process.env.CONTRACT_VERIFIER}`,
      `--verifier-url ${process.env.CONTRACT_VERIFIER_URL}`,
    ];

    if (constructorArguments) {
      command.push(`--constructor-args ${constructorArguments}`);
    }

    const { stdout, stderr } = await execAsync(command.join(" "));

    if (stderr) {
      throw new Error(stderr);
    }

    console.log(`Verification of ${contractName} successful:`, stdout);
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(
        `Error occurred during verification of ${contractName}: ${error.message}`,
      );
    } else {
      throw new Error(`Error occurred during verification of ${contractName}`);
    }
  }
}

const proverAddress = process.env.VITE_PROVER_ADDRESS;
const verifierAddress = process.env.VITE_VERIFIER_ADDRESS;

if (!proverAddress) {
  throw new Error("VITE_PROVER_ADDRESS is not set in environment variables");
}

if (!verifierAddress) {
  throw new Error("VITE_VERIFIER_ADDRESS is not set in environment variables");
}

await verifyContract(proverAddress, "WebProofProver");
await verifyContract(verifierAddress, "WebProofVerifier", `${proverAddress}`);
