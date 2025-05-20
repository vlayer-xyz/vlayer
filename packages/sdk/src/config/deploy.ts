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
import TestVerifierRouterDeployer from "../abi/TestVerifierRouterDeployer";
import { v_versions } from "../api/prover";
import type { VlayerContextConfig } from "./types";
import { AccountNotSetError } from "./errors";

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
      `Cannot get contract address from receipt: ${receipt.status}`
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
  console.log(`Prover hash: ${proverHash}`);
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

  console.log("==== Deployment Context ====");
  console.log("Account:", account.address);
  const balance = await ethClient.getBalance({ address: account.address });
  console.log("Balance (wei):", balance.toString());
  console.log("Chain:", chain.name);
  console.log("Chain ID:", chain.id);

  try {
    const gas = await ethClient.estimateGas({
      account,
      data: proverSpec.bytecode.object,
      value: 0n,
    });
    console.log("Deployment gas estimation:", gas.toString());
  } catch (err) {
    console.error("Deployment gas estimation failed:", err);
  }

  console.log("Deploying prover contract...");
  try {
    const proverSim = await ethClient.simulateContract({
      chain,
      account,
      address: "0x0000000000000000000000000000000000000000",
      abi: proverSpec.abi,
      functionName: "constructor",
      args: proverArgs,
      value: 0n,
    });
    console.log("Prover deployment simulation successful");
  } catch (err) {
    console.error("Prover deployment simulation failed:", err);
  }

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
  try {
    const verifierSim = await ethClient.simulateContract({
      chain,
      account,
      address: "0x0000000000000000000000000000000000000000",
      abi: verifierSpec.abi,
      functionName: "constructor",
      args: prover ? [prover, ...(verifierArgs ?? [])] : verifierArgs,
      value: 0n,
    });
    console.log("Verifier deployment simulation successful");
  } catch (err) {
    console.error("Verifier deployment simulation failed:", err);
  }

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
    await swapInternalVerifier(
      ethClient,
      chain,
      account,
      verifier,
      proverUrl,
      config.token
    );
  }

  return { prover, verifier };
};

const swapInternalVerifier = async (
  ethClient: EthClient,
  chain: Chain,
  account: Account,
  verifierAddress: Address,
  proverUrl: string,
  token?: string
) => {
  console.log("Swapping internal verifier");
  const imageId = await getImageId(proverUrl, token);
  const routerDeployerHash = await ethClient.deployContract({
    chain,
    account,
    args: [[imageId]],
    abi: TestVerifierRouterDeployer.abi,
    bytecode: TestVerifierRouterDeployer.bytecode.object,
  });
  const routerDeployerAddress = await waitForContractDeploy({
    client: ethClient,
    hash: routerDeployerHash,
  });
  const newVerifier = await ethClient.readContract({
    address: routerDeployerAddress,
    functionName: "VERIFIER_ROUTER",
    abi: TestVerifierRouterDeployer.abi,
  });
  const swapTxHash = await ethClient.writeContract({
    chain,
    account,
    address: verifierAddress,
    functionName: "_setTestVerifier",
    args: [newVerifier],
    abi: parseAbi(["function _setTestVerifier(address)"]),
  });
  await waitForTransactionReceipt({ client: ethClient, hash: swapTxHash });
  console.log("Internal verifier swapped successfully");
};

async function getImageId(proverUrl: string, token?: string): Promise<Hex> {
  const version = await v_versions(proverUrl, token);
  return version.call_guest_id as Hex;
}
