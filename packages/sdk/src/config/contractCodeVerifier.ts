import { getConfig } from "./getConfig";
import { createContext } from "./createContext";
import { type Address } from "viem";
import debug from "debug";

import { exec } from "child_process";
import { promisify } from "util";

const execAsync = promisify(exec);

const log = debug("vlayer:contractCodeVerifier");

export async function verifyContract(
  contractAddress: Address,
  contractName: string,
  constructorArguments?: string,
): Promise<void> {
  try {
    const config = getConfig();
    const { chain } = createContext(config);

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

    log(`Verification of ${contractName} successful:`, stdout);
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
