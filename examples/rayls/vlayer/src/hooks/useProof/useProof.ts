import { useWebProofProvider } from "./useWebProofProvider";
import { useVlayerContext } from "./useVlayerContext";
import { useVlayerFlowReducer } from "./useVlayerFlowReducer";
import { VlayerFlowActionKind, VlayerFlowStage } from "./types";
import { ExtensionMessageType, GetWebProofArgs } from "@vlayer/sdk";
import { createContext } from "@vlayer/sdk/config";
import { Abi, ContractFunctionName } from "viem";
import { useVlayerClient } from "./useVlayerClient";
import { useEffect } from "react";

export const useVlayerFlow = ({
  webProofConfig,
}: {
  webProofConfig: GetWebProofArgs<Abi, ContractFunctionName>;
}) => {
  const { stage, zkProof, webProof, verification, beauty, dispatch } =
    useVlayerFlowReducer();

  const webProofProvider = useWebProofProvider();

  useEffect(() => {
    webProofProvider.addEventListeners(
      ExtensionMessageType.ProofDone,
      ({ payload: { proof, beauty } }) => {
        dispatch({
          kind: VlayerFlowActionKind.WEB_PROOF_RECEIVED,
          payload: {
            webproof: proof,
            beauty,
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
    beauty,
    webProofProvider,
    walletClient,
    stage,
    zkProof,
    webProof,
    verification,
    vlayerClient,
    isZkProving: stage === VlayerFlowStage.ZK_PROOF_REQUESTED,
    isWebProving: stage === VlayerFlowStage.WEB_PROOF_REQUESTED,
    isVerifying: stage === VlayerFlowStage.VERIFICATION_REQUESTED,
    requestZkProof: async () => {
      dispatch({ kind: VlayerFlowActionKind.ZK_PROOF_REQUESTED });

      const zkProof = await vlayerClient.zkProve([
        {
          webProofJson: JSON.stringify({
            presentation_json: webProof,
            notary_pub_key: webProofConfig.notaryPubKey,
          }),
        },
        webProofConfig.account.address
      ]);
      console.log("zkProof", zkProof);
      dispatch({
        kind: VlayerFlowActionKind.ZK_PROOF_RECEIVED,
        payload: { zkProof },
      });
    },
    requestWebProof: () => {
      webProofProvider.requestWebProof(webProofConfig);
      dispatch({ kind: VlayerFlowActionKind.WEB_PROOF_REQUESTED });
    },
    requestVerification: async () =>{

      console.log("zkProof", zkProof);

      const { chain, ethClient, account, confirmations } =
      await createContext({
        chainName: import.meta.env.VITE_CHAIN_NAME,
        proverUrl: import.meta.env.VITE_PROVER_URL,
        jsonRpcUrl: import.meta.env.VITE_JSON_RPC_URL,
        privateKey: import.meta.env.VITE_PRIVATE_KEY,
      });

      const txHash = await ethClient.writeContract({
        address: import.meta.env.VITE_VERIFIER_ADDRESS,
        abi: webProofConfig.verifierAbi,
        functionName: "verify",
        args: zkProof,
        chain,
        account: account,
      });
  
      const verification = await ethClient.waitForTransactionReceipt({
        hash: txHash,
        confirmations,
        retryCount: 60,
        retryDelay: 1000,
      });
      console.log("Verified!", verification);

      dispatch({ kind: VlayerFlowActionKind.VERIFICATION_REQUESTED })
    },
    dispatch,
  };
};
