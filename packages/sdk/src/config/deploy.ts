import { getConfig } from "./getConfig";
import { createContext } from "./createContext";
import { type ContractArg, type ContractSpec } from "types/ethereum";
import {
  type Account,
  type Address,
  type Chain,
  type Hex,
  parseAbi,
  type PublicClient,
  type WalletClient,
} from "viem";
import { getChainConfirmations } from "./getChainConfirmations";
import debug from "debug";
import TestVerifierRouterDeployer from "../abi/TestVerifierRouterDeployer";
import type { DeployConfig } from "./types";
import { v_versions } from "../api/v_versions";

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

export const deployVlayerContracts = async ({
  proverSpec,
  verifierSpec,
  proverArgs,
  verifierArgs,
  env,
}: {
  proverSpec: ContractSpec;
  verifierSpec: ContractSpec;
  proverArgs?: ContractArg[];
  verifierArgs?: ContractArg[];
  env?: DeployConfig;
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
  if (env?.isTesting) {
    await swapInternalVerifier(ethClient, chain, account, verifier);
  }

  return { prover, verifier };
};

const swapInternalVerifier = async (
  ethClient: WalletClient & PublicClient,
  chain: Chain,
  account: Account,
  verifierAddress: Address,
) => {
  log("Swapping internal verifier");
  const imageId = await getImageId();
  const args = [
    [
      imageId,
      "0xd428ff94d9ee975e786938325e4d1cbfc05b6a8cb2daf00c17eadf07aba628ed",
    ],
  ] as const;
  console.log(args);
  const routerDeployerHash = await ethClient.deployContract({
    chain,
    account,
    args,
    abi: TestVerifierRouterDeployer.abi,
    bytecode: TestVerifierRouterDeployer.bytecode.object,
  });
  const routerDeployerAddress = await waitForContractDeploy({
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
  await waitForTransactionReceipt({ hash: swapTxHash });
  log("Internal verifier swapped successfully");
};

async function getImageId(): Promise<Hex> {
  const { proverUrl } = getConfig();
  const version = await v_versions(proverUrl);
  console.log("version", version);
  return version.result.chain_guest_id as Hex;
}
