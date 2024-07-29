import { type CallParams, type CallContext, v_call } from "./v_call";
import { encodeFunctionData, type Address, type Abi, type Hex } from "viem";
import { client } from "./helpers";


type Bytecode = {
    object: Hex,
}

export type ProverSpec = {
    abi: Abi,
    bytecode: Bytecode,
    object: Hex
}

type ProverArg = number | string | boolean;


export async function getProverSpec(file: string): Promise<ProverSpec> {
    const output: ProverSpec = await Bun.file(file).json();
    return output;
}

export async function prove(caller: Address, prover: Address, proverSpec: ProverSpec, functionName: string, args: ProverArg[], blockNo: number): Promise<any> {
    let calldata = encodeFunctionData({
        abi: proverSpec.abi,
        functionName,
        args
    });

    let call: CallParams = { caller, to: prover, data: calldata };
    let context: CallContext = { block_no: blockNo, chain_id: 1 };

    let response = await v_call(call, context); 
    if (response.result === undefined) {
      throw Error(`Server responded with error: ${JSON.stringify(response.error)}`);
    }

    return response;
}
