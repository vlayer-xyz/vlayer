import { Abi, ContractFunctionName } from "viem";
import webProofProver from "../../../out/WebProofProver.sol/WebProofProver";
import { optimismSepolia, anvil } from "viem/chains";
import { ProveArgs } from "@vlayer/sdk";
export const vlayerProverConfig: Omit<
  ProveArgs<Abi, ContractFunctionName<Abi>>,
  "args"
> = {
  address: import.meta.env.VITE_PROVER_ADDRESS as `0x${string}`,
  proverAbi: webProofProver.abi,
  chainId:
    import.meta.env.VITE_CHAIN_NAME === "anvil" ? anvil.id : optimismSepolia.id,
  functionName: "main",
  token: import.meta.env.VITE_VLAYER_API_TOKEN,
};
