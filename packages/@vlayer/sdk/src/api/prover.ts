import type { CallParams, CallContext, Json } from "./v_call";

import { v_call } from "./v_call";
import { encodeFunctionData, type Address, type Abi, type Hex } from "viem";
import { sepolia } from "viem/chains";

type Bytecode = {
    object: Hex,
}

export type ContractSpec = {
    abi: Abi,
    bytecode: Bytecode,
}

type ProverArg = number | string | boolean;


export async function getContractSpec(file: string): Promise<ContractSpec> {
    const output: ContractSpec = await Bun.file(file).json();
    return output;
}

export async function prove(prover: Address, proverSpec: ContractSpec, functionName: string, args: ProverArg[], blockNo: number = 1): Promise<Json> {
    const calldata = encodeFunctionData({
        abi: proverSpec.abi,
        functionName,
        args
    });

    const call: CallParams = { to: prover, data: calldata };
    const context: CallContext = { 
        block_no: blockNo ?? 1, //TODO: remove once backend removes this field validation
        chain_id: sepolia.id 
    };

    const response = await v_call(call, context); 
    if (response.result === undefined) {
      throw Error(`Server responded with error: ${JSON.stringify(response.error)}`);
    }

    return response;
}
