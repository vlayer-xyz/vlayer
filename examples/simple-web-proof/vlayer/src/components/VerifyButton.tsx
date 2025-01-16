import webProofProofVerifier from "../../../out/WebProofVerifier.sol/WebProofVerifier";
import { useWriteContract, useWaitForTransactionReceipt } from "wagmi";
import { Button, Loading } from "react-daisyui";
import { privateKeyToAccount } from "viem/accounts";

export function VerifyButton(props: { zkProof: unknown[] }) {
  const { writeContract, data: txHash, status, error } = useWriteContract();
  const { isLoading } = useWaitForTransactionReceipt({
    hash: txHash,
  });

  return (
    <div>
      <Button
        color="primary"
        disabled={!props.zkProof}
        onClick={() => {
          const writeContractArgs: Parameters<typeof writeContract>[0] = {
            address: import.meta.env.VITE_VERIFIER_ADDRESS as `0x${string}`,
            abi: webProofProofVerifier.abi,
            functionName: "verify",
            args: props.zkProof,
          };
          const usePrivateKey =
            !import.meta.env.VITE_USE_WINDOW_ETHEREUM_TRANSPORT &&
            import.meta.env.EXAMPLES_TEST_PRIVATE_KEY;
          if (usePrivateKey) {
            writeContract(writeContractArgs);
          } else {
            writeContract({
              ...writeContractArgs,
              account: privateKeyToAccount(
                import.meta.env.EXAMPLES_TEST_PRIVATE_KEY as `0x${string}`,
              ),
            });
          }
        }}
        className={`${!props.zkProof ? "!text-gray-400" : "hover:shadow-primary/50"}`}
      >
        <div className="flex items-center justify-center gap-2">
          {isLoading && <Loading variant="infinity" />}
          {isLoading ? "Verifying..." : "Verify Proof"}
        </div>
        {status}
      </Button>
      {error && <div>{JSON.stringify(error)}</div>}
    </div>
  );
}
