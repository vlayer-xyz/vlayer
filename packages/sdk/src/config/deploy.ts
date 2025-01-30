import { getConfig } from "./getConfig";
import { createContext } from "./createContext";
import { type ContractArg, type ContractSpec } from "types/ethereum";
import { type Address } from "viem";
import { getChainConfirmations } from "./getChainConfirmations";
import debug from "debug";
import TestVerifierRouterDeployer from "../abi/TestVerifierRouterDeployer";

const log = debug("vlayer:prover");

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

export const deployTestingVlayerContracts = async (args: {
  proverSpec: ContractSpec;
  verifierSpec: ContractSpec;
  proverArgs?: ContractArg[];
  verifierArgs?: ContractArg[];
}) => {
  const { prover, verifier } = await deployVlayerContracts(args);

  if (process.env.VLAYER_ENV !== "dev") {
    log("Not in development mode, skipping verifier router deployment");
    return { prover, verifier };
  }

  const config = getConfig();
  const { chain, ethClient, account } = createContext(config);

  log("Swapping internal verifier");
  const routedDeployerHash = await ethClient.deployContract({
    chain,
    account,
    args: [],
    abi: TestVerifierRouterDeployer.abi,
    bytecode: TestVerifierRouterDeployer.bytecode.object,
  });
  const routerDeployerAddress = await waitForContractDeploy({
    hash: routedDeployerHash,
  });
  const swapTxHash = await ethClient.writeContract({
    chain,
    account,
    address: routerDeployerAddress,
    functionName: "swapProofVerifier",
    args: [verifier],
    abi: TestVerifierRouterDeployer.abi,
  });
  await waitForTransactionReceipt({ hash: swapTxHash });

  return { prover, verifier };
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
