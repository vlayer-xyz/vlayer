import {
  type Abi,
  type Address,
  type ContractConstructorArgs,
  type ContractFunctionArgs,
  type ContractFunctionName,
  createTestClient,
  type Hex,
  http,
  HttpTransport,
  publicActions,
  walletActions,
} from "viem";
import { privateKeyToAccount, generatePrivateKey } from "viem/accounts";
import { foundry } from "viem/chains";

import type { ContractSpec, ContractArg } from "./prover";

export const testChainId1 = 55511555;
export const testChainId2 = 1114;

const rpcUrls: Map<number, HttpTransport> = new Map([
  [testChainId1, http()],
  [testChainId2, http("http://127.0.0.1:8546")],
]);

export const chainIds = [testChainId1, testChainId2];

export function client(chainId: number = testChainId1) {
  const transport = rpcUrls.get(chainId);
  if (transport == undefined) {
    throw Error(`No url for chainId ${chainId}`);
  }

  return createTestClient({
    chain: foundry,
    mode: "anvil",
    transport,
  })
    .extend(walletActions)
    .extend(publicActions);
}

export async function deployContract(
  contractSpec: ContractSpec,
  args: ContractArg[] = [],
  chainId: number = testChainId1,
): Promise<Address> {
  const ethClient = client(chainId);
  const [deployer] = await ethClient.getAddresses();

  const txHash = await ethClient.deployContract({
    abi: contractSpec.abi,
    bytecode: contractSpec.bytecode.object,
    account: deployer,
    args,
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

export async function deployProverVerifier<
  P extends Abi,
  V extends Abi,
  PArgs extends ContractConstructorArgs<P>,
  VArgs extends ContractConstructorArgs<V>,
>(
  proverSpec: DeploySpec<P>,
  verifierSpec: DeploySpec<V>,
  args: {
    prover?: PArgs;
    verifier?: Tail<VArgs>;
  } = {},
  chainId: number = testChainId1,
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
  chainId: number = testChainId1,
) {
  const ethClient = client(chainId);

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
  chainId: number = testChainId1,
) {
  const ethClient = client(chainId);
  const [account] = await ethClient.getAddresses();

  const txHash = await ethClient.writeContract({
    abi,
    address,
    functionName,
    args,
    account,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } as any);
  // TODO: fix any to viem type

  const txReceipt = await ethClient.waitForTransactionReceipt({ hash: txHash });

  if (txReceipt.status != "success") {
    throw new Error(`Transaction failed with status: ${txReceipt.status}`);
  }

  return txReceipt;
}

export const getTestAccount = () => privateKeyToAccount(generatePrivateKey());
