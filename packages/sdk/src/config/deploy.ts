import { getConfig } from "./getConfig";
import { createContext } from "./createContext";
import { type ContractArg, type ContractSpec } from "types/ethereum";
import { type Address } from "viem";
import { getChainConfirmations } from "./getChainConfirmations";
import Debug from "debug";

const log = Debug("vlayer:prover");

export const waitForContractDeploy = async ({
  hash,
}: {
  hash: `0x${string}`;
}): Promise<Address> => {
  const { ethClient: client } = createContext(getConfig());
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

export const waitForTransactionReceipt = async ({
  hash,
}: {
  hash: `0x${string}`;
}) => {
  const { ethClient } = createContext(getConfig());
  return ethClient.waitForTransactionReceipt({
    hash,
    confirmations: getChainConfirmations(ethClient.chain?.name),
    retryCount: 120,
    retryDelay: 1000,
  });
};

export const deployProver = async ({
  proverSpec,
  proverArgs,
}: {
  proverSpec: ContractSpec;
  proverArgs?: ContractArg[];
}) => {
  const config = getConfig();
  const { ethClient, account, chain } = createContext(config);

  const proverHash = await ethClient.deployContract({
    chain,
    account,
    args: proverArgs,
    abi: proverSpec.abi,
    bytecode: proverSpec.bytecode.object,
  });
  log(`Prover hash: ${proverHash}`);
  const prover = await waitForContractDeploy({ hash: proverHash });
  return prover;
};

export const deployVlayerContracts = async ({
  proverSpec,
  verifierSpec,
  proverArgs,
  verifierArgs,
}: {
  proverSpec: ContractSpec;
  verifierSpec: ContractSpec;
  proverArgs?: ContractArg[];
  verifierArgs?: ContractArg[];
}) => {
  log("Starting contract deployment process...");
  const config = getConfig();
  const { chain, ethClient, account } = createContext(config);

  log("Deploying prover contract...");
  const proverHash = await ethClient.deployContract({
    chain,
    account,
    args: proverArgs,
    abi: proverSpec.abi,
    bytecode: proverSpec.bytecode.object,
  });
  log(`Prover hash: ${proverHash}`);
  const prover = await waitForContractDeploy({ hash: proverHash });
  log(`Prover contract deployed at: ${prover}`);

  log("Deploying verifier contract...");
  const verifierHash = await ethClient.deployContract({
    chain,
    account,
    args: prover ? [prover, ...(verifierArgs ?? [])] : verifierArgs,
    abi: verifierSpec.abi,
    bytecode: verifierSpec.bytecode.object,
  });
  const verifier = await waitForContractDeploy({ hash: verifierHash });
  log(`Verifier contract deployed at: ${verifier}`);

  log("Contract deployment completed successfully");
  return { prover, verifier };
};
