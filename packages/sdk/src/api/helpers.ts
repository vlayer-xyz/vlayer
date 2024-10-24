import {
  type Abi,
  Account,
  type Address,
  type Chain,
  type ContractFunctionArgs,
  type ContractFunctionName,
  createTestClient,
  type Hex,
  http,
  HttpTransport,
  publicActions,
  PublicClient,
  walletActions,
  WriteContractParameters,
} from "viem";

import { privateKeyToAccount, generatePrivateKey } from "viem/accounts";
import { foundry } from "viem/chains";

import type { ContractSpec, ContractArg } from "types/ethereum";

const rpcUrls: Map<number, HttpTransport> = new Map([[foundry.id, http()]]);

export const chainIds = [foundry.id];

export function createAnvilClient(
  chainId: number = foundry.id,
): ReturnType<typeof walletActions> & PublicClient {
  const transport = rpcUrls.get(chainId);
  if (transport == undefined) {
    throw Error(`No url for chainId ${chainId}`);
  }

  return createTestClient({
    chain: foundry,
    mode: "anvil",
    transport: transport,
  })
    .extend(publicActions)
    .extend(walletActions);
}

export async function deployContract(
  contractSpec: ContractSpec,
  args: ContractArg[] = [],
  chainId: number = foundry.id,
): Promise<Address> {
  const ethClient = createAnvilClient(chainId);

  const [deployer] = await ethClient.getAddresses();

  const txHash = await ethClient.deployContract({
    abi: contractSpec.abi,
    bytecode: contractSpec.bytecode.object,
    account: deployer,
    args,
    chain: foundry,
  });

  const receipt = await ethClient.waitForTransactionReceipt({ hash: txHash });

  if (receipt.status != "success") {
    throw new Error(
      `Contract deployment failed with status: ${receipt.status}`,
    );
  }

  return receipt.contractAddress as Address;
}

type DeploySpec<T extends Abi> = {
  abi: T;
  bytecode: {
    object: Hex;
  };
};

type Tail<T> = T extends readonly [unknown, ...infer U] ? U : [];

export async function deployProverVerifier<P extends Abi, V extends Abi>(
  proverSpec: DeploySpec<P>,
  verifierSpec: DeploySpec<V>,
  args: {
    prover?: ContractArg[];
    verifier?: Tail<ContractArg>[];
  } = {},
  chainId: number = foundry.id,
) {
  console.log("Deploying prover");
  const proverAddress = await deployContract(
    proverSpec,
    args.prover ?? [],
    chainId,
  );
  console.log(`Prover has been deployed on ${proverAddress} address`);

  console.log("Deploying verifier");
  const verifierAddress = await deployContract(
    verifierSpec,
    [proverAddress, ...(args.verifier ?? [])],
    chainId,
  );
  console.log(`Verifier has been deployed on ${verifierAddress} address`);

  return [proverAddress, verifierAddress];
}

export async function call<
  T extends Abi,
  F extends ContractFunctionName<T, "pure" | "view">,
>(
  abi: T,
  address: Address,
  functionName: F,
  args?: ContractFunctionArgs<T, "pure" | "view", F>,
  chainId: number = foundry.id,
) {
  const ethClient = createAnvilClient(chainId);

  return ethClient.readContract({
    abi,
    address,
    functionName,
    args,
  });
}

export async function writeContract<
  T extends Abi,
  F extends ContractFunctionName<T, "payable" | "nonpayable">,
>(
  address: Address,
  abi: T,
  functionName: F,
  args: ContractFunctionArgs<T, "payable" | "nonpayable", F>,
  sender?: Address,
  chain: Chain = foundry,
) {
  const ethClient = createAnvilClient(chain.id);
  const selectedSender = sender || (await ethClient.getAddresses())[0];

  const txHash = await ethClient.writeContract({
    address,
    abi: abi as Abi,
    functionName,
    args: args as readonly unknown[],
    chain,
    account: selectedSender,
    chainOverride: undefined,
  } as WriteContractParameters<
    Abi,
    ContractFunctionName<Abi, "nonpayable" | "payable">,
    ContractFunctionArgs<Abi, "nonpayable" | "payable", F>,
    Chain | undefined,
    Account | undefined,
    Chain | undefined
  >);

  const txReceipt = await ethClient.waitForTransactionReceipt({ hash: txHash });

  if (txReceipt.status != "success") {
    throw new Error(`Transaction failed with status: ${txReceipt.status}`);
  }

  return txReceipt;
}

export const getTestAccount = () => privateKeyToAccount(generatePrivateKey());

export const getTestAddresses = (
  chainId: number = foundry.id,
): Promise<Address[]> => createAnvilClient(chainId).getAddresses();
