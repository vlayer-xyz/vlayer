import { createTestClient, walletActions, publicActions, http, Address } from "viem";
import { foundry } from "viem/chains";
import { type ProverSpec } from "./prover";

export function client() {
    return createTestClient({
        chain: foundry,
        mode: 'anvil',
        transport: http()
    })
        .extend(walletActions)
        .extend(publicActions);
}

export async function deployProver(proverSpec: ProverSpec): Promise<Address> {
    const ethClient = client();
    const [deployer] = await ethClient.getAddresses();

    const txHash = await ethClient.deployContract({
        abi: proverSpec.abi,
        bytecode: proverSpec.bytecode.object,
        account: deployer
    });

    const receipt = await ethClient.waitForTransactionReceipt({ hash: txHash })

    let adddress = receipt.contractAddress;

    if (adddress === undefined || adddress === null) {
        throw new Error(`Contract deployment failed with status: ${receipt.status}`);
    }

    return adddress;
}
