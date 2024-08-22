import type { ContractSpec } from "./prover";
import type { Address, HttpTransport } from "viem";

import { createTestClient, walletActions, publicActions, http } from "viem";
import { privateKeyToAccount, generatePrivateKey } from 'viem/accounts'
import { foundry } from "viem/chains";
import { type ContractSpec } from "./prover";

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

    const receipt = await ethClient.waitForTransactionReceipt({ hash: txHash })

    let adddress = receipt.contractAddress;

    if (adddress === undefined || adddress === null) {
        throw new Error(`Contract deployment failed with status: ${receipt.status}`);
    }

    return adddress;
}

export const getTestAccount = () => privateKeyToAccount(generatePrivateKey());