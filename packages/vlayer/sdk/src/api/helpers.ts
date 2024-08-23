import {
  Abi,
  Address,
  ContractFunctionArgs,
  ContractFunctionName,
  createTestClient,
  http,
  HttpTransport,
  publicActions,
  walletActions
} from "viem";
import { privateKeyToAccount, generatePrivateKey } from 'viem/accounts'
import { foundry } from "viem/chains";

import type { ContractSpec } from "./prover";

export const testChainId1 = 55511555;
export const testChainId2 = 1114;
const rpcUrls: Map<number, HttpTransport> = new Map([[testChainId1, http()], [testChainId2, http("http://127.0.0.1:8546")]]);

export function client(chainId: number = testChainId1) {
    let transport = rpcUrls.get(chainId);
    if (transport == undefined) {
        throw Error(`No url for chainId ${chainId}`);
    }
    
    return createTestClient({
        chain: foundry,
        mode: 'anvil',
        transport
    })
        .extend(walletActions)
        .extend(publicActions);
}

export async function deployContract(contractSpec: ContractSpec, args: any[] = [], chainId: number = testChainId1): Promise<Address> {
    const ethClient = client(chainId);
    const [deployer] = await ethClient.getAddresses();

  const txHash = await ethClient.deployContract({
    abi: contractSpec.abi,
    bytecode: contractSpec.bytecode.object,
    account: deployer,
    args
  });

  const receipt = await ethClient.waitForTransactionReceipt({hash: txHash})

  let adddress = receipt.contractAddress;

  if (adddress === undefined || adddress === null) {
    throw new Error(`Contract deployment failed with status: ${receipt.status}`);
  }

  return adddress;
}


export async function call<T extends Abi, F extends ContractFunctionName<T, 'pure' | 'view'>>(abi: T, address: Address, functionName: F, args?: ContractFunctionArgs<T, 'pure' | 'view', F>, chainId: number = testChainId1) {
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
  F extends ContractFunctionName<T, 'payable' | 'nonpayable'>,
>(address: Address, abi: T, functionName: F, args: ContractFunctionArgs<T, 'payable' | 'nonpayable', F>, chainId: number = testChainId1) {
  const ethClient = client(chainId);
  const [account] = await ethClient.getAddresses();

  const txHash = await ethClient.writeContract({
    abi,
    address,
    functionName,
    args,
    account,
  } as any);

  return ethClient.waitForTransactionReceipt({hash: txHash});
}

export const getTestAccount = () => privateKeyToAccount(generatePrivateKey());
