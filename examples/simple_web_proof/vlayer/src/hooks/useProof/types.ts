import { Transaction } from "viem";

export enum VlayerFlowStage {
  INITIAL = "ready",
  WEB_PROOF_REQUESTED = "web_proof_requested",
  WEB_PROOF_RECEIVED = "web_proof_received",
  ZK_PROOF_REQUESTED = "zk_proof_requested",
  ZK_PROOF_RECEIVED = "zk_proof_received",
  VERIFICATION_REQUESTED = "verification_requested",
  VERIFICATION_RECEIVED = "verification_received",
  VERIFICATION_FAILED = "verification_failed",
}

export enum VlayerFlowActionKind {
  WEB_PROOF_REQUESTED = "web_proof_requested",
  WEB_PROOF_RECEIVED = "web_proof_received",
  ZK_PROOF_REQUESTED = "zk_proof_requested",
  ZK_PROOF_RECEIVED = "zk_proof_received",
  VERIFICATION_REQUESTED = "verification_requested",
  VERIFICATION_RECEIVED = "verification_received",
  VERIFICATION_FAILED = "verification_failed",
}

export type VlayerFlowAction =
  | {
      kind: VlayerFlowActionKind.WEB_PROOF_REQUESTED;
    }
  | {
      kind: VlayerFlowActionKind.WEB_PROOF_RECEIVED;
      payload: {
        webproof: unknown;
      };
    }
  | {
      kind: VlayerFlowActionKind.ZK_PROOF_REQUESTED;
    }
  | {
      kind: VlayerFlowActionKind.ZK_PROOF_RECEIVED;
      payload: {
        zkProof: unknown;
      };
    }
  | {
      kind: VlayerFlowActionKind.VERIFICATION_REQUESTED;
    }
  | {
      kind: VlayerFlowActionKind.VERIFICATION_RECEIVED;
      payload: {
        verification: unknown;
      };
    }
  | {
      kind: VlayerFlowActionKind.VERIFICATION_FAILED;
      payload: {
        error: string | undefined;
      };
    };

export type VlayerFlowState = {
  stage: VlayerFlowStage;
  zkProof: unknown;
  webProof: unknown;
  verification: Transaction;
};
