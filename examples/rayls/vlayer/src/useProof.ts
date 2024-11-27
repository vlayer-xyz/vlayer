import { useMemo, useReducer, useState } from "react";

import { createExtensionWebProofProvider } from "@vlayer/sdk/web_proof";

import { createContext } from "@vlayer/sdk/config";

import { customTransport } from "@vlayer/sdk/config";

import { match } from "ts-pattern";

enum VlayerFlowStage {
  INITIAL = "ready",
  WEB_PROOF_REQUESTED = "web_proof_requested",
  WEB_PROOF_RECEIVED = "web_proof_received",
  ZK_PROOF_REQUESTED = "zk_proof_requested",
  ZK_PROOF_RECEIVED = "zk_proof_received",
  VERIFICATION_REQUESTED = "verification_requested",
  VERIFICATION_RECEIVED = "verification_received",
}

enum VlayerFlowActionKind {
  WEB_PROOF_REQUESTED = "web_proof_requested",
  WEB_PROOF_RECEIVED = "web_proof_received",
  ZK_PROOF_REQUESTED = "zk_proof_requested",
  ZK_PROOF_RECEIVED = "zk_proof_received",
  VERIFICATION_REQUESTED = "verification_requested",
  VERIFICATION_RECEIVED = "verification_received",
}

type VlayerFlowAction =
  | {
      kind: VlayerFlowActionKind.WEB_PROOF_REQUESTED;
      payload: never;
    }
  | {
      kind: VlayerFlowActionKind.WEB_PROOF_RECEIVED;
      payload: {
        webproof: unknown;
      };
    }
  | {
      kind: VlayerFlowActionKind.ZK_PROOF_REQUESTED;
      payload: never;
    }
  | {
      kind: VlayerFlowActionKind.ZK_PROOF_RECEIVED;
      payload: {
        zkProof: unknown;
      };
    }
  | {
      kind: VlayerFlowActionKind.VERIFICATION_REQUESTED;
      payload: never;
    }
  | {
      kind: VlayerFlowActionKind.VERIFICATION_RECEIVED;
      payload: {
        verification: unknown;
      };
    };

type VlayerFlowState = {
  stage: VlayerFlowStage;
  zkProof: unknown;
  webProof: unknown;
  verification: unknown;
};

const vlayerFlowReducer = (prev: VlayerFlowState, action: VlayerFlowAction) => {
  return match(action)
    .with({ kind: VlayerFlowActionKind.WEB_PROOF_REQUESTED }, ({}) => {
      return {
        ...prev,
        stage: VlayerFlowStage.WEB_PROOF_REQUESTED,
      };
    })
    .with({ kind: VlayerFlowActionKind.WEB_PROOF_RECEIVED }, ({ payload }) => {
      return {
        ...prev,
        stage: VlayerFlowStage.WEB_PROOF_RECEIVED,
        webProof: payload.webproof,
      };
    })
    .with({ kind: VlayerFlowActionKind.ZK_PROOF_REQUESTED }, () => {
      return {
        ...prev,
        stage: VlayerFlowStage.ZK_PROOF_REQUESTED,
      };
    })
    .with({ kind: VlayerFlowActionKind.ZK_PROOF_RECEIVED }, ({ payload }) => {
      return {
        ...prev,
        stage: VlayerFlowStage.ZK_PROOF_RECEIVED,
        zkProof: payload.zkProof,
      };
    })
    .with({ kind: VlayerFlowActionKind.VERIFICATION_REQUESTED }, () => {
      return {
        ...prev,
        stage: VlayerFlowStage.VERIFICATION_REQUESTED,
      };
    })
    .with(
      { kind: VlayerFlowActionKind.VERIFICATION_RECEIVED },
      ({ payload }) => {
        return {
          ...prev,
          stage: VlayerFlowStage.VERIFICATION_RECEIVED,
          verification: payload.verification,
        };
      },
    )
    .exhaustive();
};

export const useVlayerFlow = () => {
  const [stade, dispatch] = useReducer(
    (prev: VlayerFlowStage, next: VlayerFlowStage) => next,
    VlayerFlowStage.INITIAL,
  );

  const { ethClient: walletClient } = createContext(
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
  const webProofProvider = useMemo(
    () =>
      createExtensionWebProofProvider({
        notaryUrl: import.meta.env.VITE_NOTARY_URL,
        wsProxyUrl: import.meta.env.VITE_WS_PROXY_URL,
      }),
    [],
  );
  return {
    webProofProvider,
    walletClient,
  };
};
