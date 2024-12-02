import Menu from "../components/Menu";
import { config } from "../config";
import { useVlayerFlow } from "../hooks/useProof";

export default function Actions() {
  const vlayerFlow = useVlayerFlow({
    webProofConfig: config,
  });

  return <Menu vlayerFlow={vlayerFlow} />;
}
