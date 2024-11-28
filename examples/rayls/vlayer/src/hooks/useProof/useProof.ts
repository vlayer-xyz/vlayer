import { useWebProofProvider } from "./useWebProofProvider";
import { useVlayerContext } from "./useVlayerContext";
import { useVlayerFlowReducer } from "./useVlayerFlowReducer";
import { VlayerFlowActionKind, VlayerFlowStage } from "./types";
import { ExtensionMessageType, GetWebProofArgs } from "@vlayer/sdk";
import { Abi, ContractFunctionName } from "viem";
import { useVlayerClient } from "./useVlayerClient";
import { useEffect } from "react";

export const useVlayerFlow = ({
  webProofConfig,
}: {
  webProofConfig: GetWebProofArgs<Abi, ContractFunctionName>;
}) => {
  const { stage, zkProof, webProof, verification, dispatch } =
    useVlayerFlowReducer();

  const webProofProvider = useWebProofProvider();

  useEffect(() => {
    webProofProvider.addEventListeners(
      ExtensionMessageType.ProofDone,
      ({ payload: { proof } }) => {
        dispatch({
          kind: VlayerFlowActionKind.WEB_PROOF_RECEIVED,
          payload: {
            webproof: proof,
          },
        });
      },
    );
  }, []);

  const walletClient = useVlayerContext();
  const vlayerClient = useVlayerClient(
    webProofConfig.proverCallCommitment.proverAbi,
    webProofConfig.proverCallCommitment.chainId,
  );
  return {
    webProofProvider,
    walletClient,
    stage,
    zkProof,
    webProof,
    verification,
    vlayerClient,
    isZkProving: stage === VlayerFlowStage.VERIFICATION_REQUESTED,
    isWebProving: stage === VlayerFlowStage.WEB_PROOF_REQUESTED,
    isVerifying: stage === VlayerFlowStage.VERIFICATION_REQUESTED,
    requestZkProof: () => {
      console.log("Requesting zk proof with web proof", webProof);
      vlayerClient.zkProve([
        {
          webProofJson: JSON.stringify({
            presentation_json: webProof,
            notary_pub_key: webProofConfig.notaryPubKey,
          }),
        },
      ]);
      dispatch({ kind: VlayerFlowActionKind.ZK_PROOF_REQUESTED });
    },
    requestWebProof: () => {
      webProofProvider.requestWebProof(webProofConfig);
      dispatch({ kind: VlayerFlowActionKind.WEB_PROOF_REQUESTED });
    },
    requestVerification: () =>
      dispatch({ kind: VlayerFlowActionKind.VERIFICATION_REQUESTED }),
    dispatch,
  };
};
