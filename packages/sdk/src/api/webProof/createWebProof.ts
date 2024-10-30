import {
  GetWebProofArgs,
  WebProofSetup,
} from "../../api/lib/types/webProofProvider";
import { Abi, ContractFunctionName } from "viem";

export function createWebProof<
  T extends Abi,
  F extends ContractFunctionName<T>,
>(args: GetWebProofArgs<T, F>): WebProofSetup {
  return {
    logoUrl: args.logoUrl,
    steps: args.steps,
    isWebProof: true,
  } as WebProofSetup;
}
