import { useMemo, useEffect, useState } from "react";
import { BrandedHash, createVlayerClient } from "@vlayer/sdk";

import {
  type Address,
  type Abi,
  ContractFunctionName,
  ContractFunctionReturnType,
  AbiStateMutability,
  ContractFunctionArgs,
} from "viem";

import { createWebProofProvider, createContext } from "@vlayer/sdk"; 


export const useProof = () => {
    const { ethClient: walletClient } = await createContext(
        {
          chainName: import.meta.env.VITE_CHAIN_NAME as string,
          proverUrl: import.meta.env.VITE_PROVER_URL as string,
          jsonRpcUrl: import.meta.env.VITE_JSON_RPC_URL as string,
          privateKey: import.meta.env.VITE_PRIVATE_KEY as `0x${string}`,
        },
        import.meta.env.VITE_USE_WINDOW_ETHEREUM_TRANSPORT
          ? customTransport(window.ethereum)
          : undefined,
      );
    const webProofProvider = useMemo(() => createWebProofProvider(import.meta.env.VITE_PROVER_URL),
    [import.meta.env.VITE_PROVER_URL]

)
// interface UseProverParams<T extends Abi, F extends ContractFunctionName<T>> {
//   addr: Address;
//   abi: T;
//   func: F;
//   chainId: number;
// }
// interface UseProverReturn<T extends Abi, F extends ContractFunctionName<T>> {
//   prove: (
//     proverArgs: ContractFunctionArgs<T, AbiStateMutability, F>,
//   ) => Promise<BrandedHash<T, F> | undefined>;
//   provingError: string | null;
//   proof: ContractFunctionReturnType<T, AbiStateMutability, F> | null;
// }

// const useProver = <T extends Abi, F extends ContractFunctionName<T>>({
//   addr,
//   abi,
//   func,
//   chainId,
// }: UseProverParams<T, F>): UseProverReturn<T, F> => {
//   const [provingHash, setProvingHash] = useState<BrandedHash<T, F> | null>(
//     null,
//   );
//   const [provingError, setProvingError] = useState<string | null>(null);
//   const [proof, setProof] = useState<ContractFunctionReturnType<
//     T,
//     AbiStateMutability,
//     F
//   > | null>(null);

//   const vlayer = useMemo(
//     () =>
//       createVlayerClient({
//         url: import.meta.env.VITE_PROVER_URL,
//       }),
//     [import.meta.env.VITE_PROVER_URL],
//   );

//   const prove = async (
//     args: ContractFunctionArgs<T, AbiStateMutability, F>,
//   ) => {
//     try {
//       const hash = await vlayer.prove({
//         address: addr,
//         proverAbi: abi,
//         functionName: func,
//         args,
//         chainId,
//       });

//       setProvingHash(hash);

//       return hash;
//     } catch (err) {
//       setProvingError("Cannot start proving, check logs");
//       console.error(err);
//     }
//   };

//   useEffect(() => {
//     if (provingHash) {
//       const waitForProof = async () => {
//         console.log("Waiting for proving result: ", provingHash);
//         const result = await vlayer.waitForProvingResult(provingHash);
//         setProof(result);
//         console.log("Proof ready:", result);
//       };

//       waitForProof().catch((err) => {
//         setProvingError("Cannot finalize proving, check logs");
//         console.error(err);
//       });
//     }
//   }, [provingHash]);

//   return {
//     prove,
//     provingError,
//     proof,
//   };
};

export default useProver;
