import { createTestClient, walletActions, publicActions, http, Address } from "viem";
import { foundry } from "viem/chains";
import { type ContractSpec } from "./prover";

export function client() {
    return createTestClient({
        chain: foundry,
        mode: 'anvil',
        transport: http()
    })
        .extend(walletActions)
        .extend(publicActions);
}

export async function deployContract(contractSpec: ContractSpec, args: any[] = []): Promise<Address> {
    const ethClient = client();
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
