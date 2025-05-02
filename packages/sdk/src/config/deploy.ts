import { getConfig } from "./getConfig";
import { createContext, type EthClient } from "./createContext";
import { type ContractArg, type ContractSpec } from "types/ethereum";
import {
  type Account,
  type Address,
  type Chain,
  type Hex,
  parseAbi,
} from "viem";
import { getChainConfirmations } from "./utils/getChainConfirmations";
import debug from "debug";
import TestVerifierRouterDeployer from "../abi/TestVerifierRouterDeployer";
import { v_versions } from "../api/v_versions";
import type { VlayerContextConfig } from "./types";
import { AccountNotSetError } from "./errors";

const log = debug("vlayer:prover");

export const waitForContractDeploy = async ({
  client,
  hash,
}: {
  client: EthClient;
  hash: `0x${string}`;
}): Promise<Address> => {
  const receipt = await waitForTransactionReceipt({ client, hash });

  if (!receipt.contractAddress || receipt.status !== "success") {
    throw new Error(
      `Cannot get contract address from receipt: ${receipt.status}`,
    );
  }

  return receipt.contractAddress;
};

export const waitForTransactionReceipt = async ({
  client,
  hash,
}: {
  client: EthClient;
  hash: `0x${string}`;
}) => {
  return client.waitForTransactionReceipt({
    hash,
    confirmations: getChainConfirmations(client.chain?.name),
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
  if (!account) {
    throw new AccountNotSetError();
  }
  const proverHash = await ethClient.deployContract({
    chain,
    account,
    args: proverArgs,
    abi: proverSpec.abi,
    bytecode: proverSpec.bytecode.object,
  });
  log(`Prover hash: ${proverHash}`);
  const prover = await waitForContractDeploy({
    client: ethClient,
    hash: proverHash,
  });
  return prover;
};

export const deployVlayerContracts = async ({
  proverSpec,
  verifierSpec,
  proverArgs,
  verifierArgs,
  config: configOverride,
}: {
  proverSpec: ContractSpec;
  verifierSpec: ContractSpec;
  proverArgs?: ContractArg[];
  verifierArgs?: ContractArg[];
  config?: Partial<VlayerContextConfig>;
}) => {
  console.log("Starting contract deployment process...");
  const config = getConfig(configOverride);
  const { chain, ethClient, account, deployConfig, proverUrl } =
    createContext(config);
  if (!account) {
    throw new AccountNotSetError();
  }
  console.log("Deploying prover contract...");
  const proverHash = await ethClient.deployContract({
    chain,
    account,
    args: proverArgs,
    abi: proverSpec.abi,
    bytecode: proverSpec.bytecode.object,
  });
  console.log(`Prover hash: ${proverHash}`);
  const prover = await waitForContractDeploy({
    client: ethClient,
    hash: proverHash,
  });
  console.log(`Prover contract deployed at: ${prover}`);

  console.log("Deploying verifier contract...");
  const verifierHash = await ethClient.deployContract({
    chain,
    account,
    args: prover ? [prover, ...(verifierArgs ?? [])] : verifierArgs,
    abi: verifierSpec.abi,
    bytecode: verifierSpec.bytecode.object,
  });
  const verifier = await waitForContractDeploy({
    client: ethClient,
    hash: verifierHash,
  });
  console.log(`Verifier contract deployed at: ${verifier}`);

  console.log("Contract deployment completed successfully");
  if (deployConfig.shouldRedeployVerifierRouter) {
    console.log("Redeploying verifier router...");
    await swapInternalVerifier(
      ethClient,
      chain,
      account,
      verifier,
      proverUrl,
      config.token,
    );
  }

  console.log("Redeploying verifier router completed successfully");
  return { prover, verifier };
};

const swapInternalVerifier = async (
  ethClient: EthClient,
  chain: Chain,
  account: Account,
  verifierAddress: Address,
  proverUrl: string,
  token?: string,
) => {
  console.log("Starting the process to swap internal verifier...");
  console.log(`Prover URL: ${proverUrl}`);
  console.log(`Verifier Address: ${verifierAddress}`);
  
  const imageId = await getImageId(proverUrl, token);
  console.log(`Retrieved Image ID: ${imageId}`);
  
  console.log("Deploying TestVerifierRouterDeployer contract...");
  const routerDeployerHash = await ethClient.deployContract({
    chain,
    account,
    args: [[imageId]],
    abi: TestVerifierRouterDeployer.abi,
    bytecode: TestVerifierRouterDeployer.bytecode.object,
  });
  console.log(`Router Deployer Transaction Hash: ${routerDeployerHash}`);
  
  const routerDeployerAddress = await waitForContractDeploy({
    client: ethClient,
    hash: routerDeployerHash,
  });
  console.log(`Router Deployer Contract Address: ${routerDeployerAddress}`);
  
  console.log("Reading new verifier address from Router Deployer contract...");
  const newVerifier = await ethClient.readContract({
    address: routerDeployerAddress,
    functionName: "VERIFIER_ROUTER",
    abi: TestVerifierRouterDeployer.abi,
  });
  console.log(`New Verifier Address: ${newVerifier}`);
  
  console.log("Writing new verifier address to the verifier contract...");
  const swapTxHash = await ethClient.writeContract({
    chain,
    account,
    address: verifierAddress,
    functionName: "_setTestVerifier",
    args: [newVerifier],
    abi: parseAbi(["function _setTestVerifier(address)"]),
  });
  console.log(`Swap Transaction Hash: ${swapTxHash}`);
  
  console.log("Waiting for transaction receipt...");
  await waitForTransactionReceipt({ client: ethClient, hash: swapTxHash });
  console.log("Internal verifier swapped successfully");
};

async function getImageId(proverUrl: string, token?: string): Promise<Hex> {
  const version = await v_versions(proverUrl, token);
  return version.call_guest_id as Hex;
}
