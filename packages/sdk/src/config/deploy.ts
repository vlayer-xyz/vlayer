import { getConfig } from "./getConfig";
import { createContext } from "./createContext";
import { ContractSpec } from "types/ethereum";
import { Address, PublicClient } from "viem";
import { getChainConfirmations } from "./getChainConfirmations";

export const waitForContracDeploy = async (
  client: PublicClient,
  hash: `0x${string}`,
): Promise<Address> => {
  console.log(`Waiting for contract deployment with hash: ${hash}`);
  const receipt = await client.waitForTransactionReceipt({
    hash,
    confirmations: getChainConfirmations(client.chain?.name),
    retryCount: 120,
    retryDelay: 1000,
  });

  if (!receipt.contractAddress || receipt.status !== "success") {
    throw new Error(
      `Cannot get contract address from receipt: ${receipt.status}`,
    );
  }

  return receipt.contractAddress;
};

export const deploy = async ({
  proverSpec,
  verifierSpec,
}: {
  proverSpec: ContractSpec;
  verifierSpec: ContractSpec;
}) => {
  console.log("Starting contract deployment process...");
  const config = getConfig();
  const { chain, ethClient, account } = await createContext(config);

  console.log("Deploying prover contract...");
  const proverHash = await ethClient.deployContract({
    chain,
    account,
    args: [],
    abi: proverSpec.abi,
    bytecode: proverSpec.bytecode.object,
  });
  const prover = await waitForContracDeploy(ethClient, proverHash);
  console.log(`Prover contract deployed at: ${prover}`);

  console.log("Deploying verifier contract...");
  const verifierHash = await ethClient.deployContract({
    chain,
    account,
    args: [prover],
    abi: verifierSpec.abi,
    bytecode: verifierSpec.bytecode.object,
  });
  const verifier = await waitForContracDeploy(ethClient, verifierHash);
  console.log(`Verifier contract deployed at: ${verifier}`);

  console.log("Contract deployment completed successfully");
  return { prover, verifier };
};
