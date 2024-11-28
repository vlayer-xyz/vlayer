import { useWebProofProvider } from "./useWebProofProvider";
import { useVlayerContext } from "./useVlayerContext";
import { useVlayerFlowReducer } from "./useVlayerFlowReducer";
import { VlayerFlowActionKind, VlayerFlowStage } from "./types";
import { GetWebProofArgs } from "@vlayer/sdk";
import { Abi, ContractFunctionName } from "viem";

export const useVlayerFlow = ({
  webProofConfig,
}: {
  webProofConfig: GetWebProofArgs<Abi, ContractFunctionName>;
}) => {
  const { stage, zkProof, webProof, verification, dispatch } =
    useVlayerFlowReducer();

  const webProofProvider = useWebProofProvider();
  const walletClient = useVlayerContext();
  console.log("webProofProvider", webProofProvider);
  return {
    webProofProvider,
    walletClient,
    stage,
    zkProof,
    webProof,
    verification,
    isZkProving: stage === VlayerFlowStage.VERIFICATION_REQUESTED,
    isWebProving: stage === VlayerFlowStage.WEB_PROOF_REQUESTED,
    isVerifying: stage === VlayerFlowStage.VERIFICATION_REQUESTED,
    requestZkProof: () => {
      dispatch({ kind: VlayerFlowActionKind.ZK_PROOF_REQUESTED });
    },
    requestWebProof: () => {
      console.log("requestZkProof");
      webProofProvider.requestWebProof(webProofConfig);
      dispatch({ kind: VlayerFlowActionKind.WEB_PROOF_REQUESTED });
    },
    requestVerification: () =>
      dispatch({ kind: VlayerFlowActionKind.VERIFICATION_REQUESTED }),
    dispatch,
  };
};
