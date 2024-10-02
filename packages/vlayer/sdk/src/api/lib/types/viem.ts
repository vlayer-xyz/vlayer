import { Abi, ContractFunctionName } from "viem";
import { AbiParametersToPrimitiveTypes, ExtractAbiFunction } from "abitype";

type Without<T extends readonly unknown[], P> = T extends readonly [
  infer F,
  ...infer R,
]
  ? F extends P
    ? Without<R, P>
    : readonly [F, ...Without<R, P>]
  : [];

export type ContractFunctionArgsWithout<
  abi extends Abi,
  functionName extends ContractFunctionName<abi>,
  without,
> =
  AbiParametersToPrimitiveTypes<
    Without<
      ExtractAbiFunction<abi extends Abi ? abi : Abi, functionName>["inputs"],
      without
    >,
    "inputs"
  > extends infer args
    ? [args] extends [never]
      ? readonly unknown[]
      : args
    : readonly unknown[];
