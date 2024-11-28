import { Button } from "react-daisyui";
import { useVlayerFlow } from "./hooks/useProof";
import { config } from "./config";
function App() {
  const vlayerFlow = useVlayerFlow({
    webProofConfig: config,
  });

  return (
    <div className="flex justify-center mt-48">
      <Button
        color="primary"
        onClick={() => {
          console.log("clicked");
          vlayerFlow.requestWebProof();
        }}
      >
        {vlayerFlow.isWebProving ? "Web Proofing..." : "Request Web Proof"}
      </Button>

      {vlayerFlow.webProof ? (
        <Button onClick={() => vlayerFlow.requestZkProof()}>
          {vlayerFlow.isZkProving ? "ZK Proofing..." : "Request ZK Proof"}
        </Button>
      ) : null}
    </div>
  );
}

export default App;
