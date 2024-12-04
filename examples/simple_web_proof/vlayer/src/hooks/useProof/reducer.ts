import { match } from "ts-pattern";
import {
  VlayerFlowStage,
  type VlayerFlowState,
  type VlayerFlowAction,
  VlayerFlowActionKind,
} from "./types";

const handleWebProofRequested = (prev: VlayerFlowState) => ({
  ...prev,
  stage: VlayerFlowStage.WEB_PROOF_REQUESTED,
});

const handleWebProofReceived = (
  prev: VlayerFlowState,
  { webproof }: { webproof: unknown },
) => ({
  ...prev,
  stage: VlayerFlowStage.WEB_PROOF_RECEIVED,
  webProof: webproof,
});

const handleZkProofRequested = (prev: VlayerFlowState) => ({
  ...prev,
  stage: VlayerFlowStage.ZK_PROOF_REQUESTED,
});

const handleZkProofReceived = (
  prev: VlayerFlowState,
  { zkProof }: { zkProof: unknown },
) => ({
  ...prev,
  stage: VlayerFlowStage.ZK_PROOF_RECEIVED,
  zkProof,
});

const handleVerificationRequested = (prev: VlayerFlowState) => ({
  ...prev,
  stage: VlayerFlowStage.VERIFICATION_REQUESTED,
});

const handleVerificationFailed = (
  prev: VlayerFlowState,
  { error }: { error: string | undefined },
) => ({
  ...prev,
  stage: VlayerFlowStage.VERIFICATION_FAILED,
  error,
});

const handleVerificationReceived = (
  prev: VlayerFlowState,
  { verification }: { verification: unknown },
) => ({
  ...prev,
  stage: VlayerFlowStage.VERIFICATION_RECEIVED,
  verification,
});

export const vlayerFlowReducer = (
  prev: VlayerFlowState,
  action: VlayerFlowAction,
) =>
  match(action)
    .with({ kind: VlayerFlowActionKind.WEB_PROOF_REQUESTED }, () =>
      handleWebProofRequested(prev),
    )
    .with({ kind: VlayerFlowActionKind.WEB_PROOF_RECEIVED }, ({ payload }) =>
      handleWebProofReceived(prev, payload),
    )
    .with({ kind: VlayerFlowActionKind.ZK_PROOF_REQUESTED }, () =>
      handleZkProofRequested(prev),
    )
    .with({ kind: VlayerFlowActionKind.ZK_PROOF_RECEIVED }, ({ payload }) =>
      handleZkProofReceived(prev, payload),
    )
    .with({ kind: VlayerFlowActionKind.VERIFICATION_REQUESTED }, () =>
      handleVerificationRequested(prev),
    )
    .with({ kind: VlayerFlowActionKind.VERIFICATION_RECEIVED }, ({ payload }) =>
      handleVerificationReceived(prev, payload),
    )
    .with({ kind: VlayerFlowActionKind.VERIFICATION_FAILED }, ({ payload }) =>
      handleVerificationFailed(prev, payload),
    )
    .exhaustive();
