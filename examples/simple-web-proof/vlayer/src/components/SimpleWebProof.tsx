import {
  useCallProver,
  useWaitForProvingResult,
  useWebProof,
} from "@vlayer/react";

import { webProofConfig } from "./webProofConfig";
import { vlayerProverConfig } from "./vlayerProverConfig";
import { CallProverButton } from "./CallProverButton";
import { RequestWebProofButton } from "./RequestWebProofButton";
import { Container } from "./Container";
import { AddressArea } from "./AddressArea";
import { useAccount } from "wagmi";
import { VerifyButton } from "./VerifyButton";
import { Buffer } from "buffer";
window.Buffer = Buffer;

export default function SimpleWebProof() {
  const {
    requestWebProof,
    webProof,
    isPending: isWebProofPending,
  } = useWebProof(webProofConfig);

  const {
    callProver,
    isPending: isCallProverPending,
    data: hash,
  } = useCallProver(vlayerProverConfig);

  const { isPending: isWaitingForProvingResult, data: result } =
    useWaitForProvingResult(hash);
  const { address } = useAccount();

  return (
    <Container>
      <RequestWebProofButton
        onClick={requestWebProof}
        isLoading={isWebProofPending}
        hasWebProof={!!webProof}
      />
      <CallProverButton
        disabled={!webProof}
        onClick={() => callProver([webProof, address])}
        isLoading={isCallProverPending || isWaitingForProvingResult}
      />
      <VerifyButton zkProof={result as string} />
      <AddressArea />
    </Container>
  );
}
