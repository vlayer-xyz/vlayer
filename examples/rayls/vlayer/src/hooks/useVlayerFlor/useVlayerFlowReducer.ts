import { useReducer } from "react";

import { vlayerFlowReducer } from "./reducer";
import { VlayerFlowStage } from "./types";

export const useVlayerFlowReducer = () => {
  const [state, dispatch] = useReducer(vlayerFlowReducer, {
    stage: VlayerFlowStage.INITIAL,
    zkProof: undefined,
    webProof: undefined,
    verification: undefined,
    beauty: undefined,
  });

  return {
    ...state,
    dispatch,
  };
};
