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
        Request Web Proof
      </Button>
    </div>
  );
}

export default App;
