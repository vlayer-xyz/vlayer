import { Button } from "react-daisyui";
import { useVlayerFlow } from "./hooks/useProof";
import { config } from "./config";
function App() {
  const vlayerFlow = useVlayerFlow({
    webProofConfig: config,
  });

  console.log("beauty", vlayerFlow.beauty);
  return (
    <div className="flex flex-col items-center gap-4 mt-48">
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
        <>
          <Button color="secondary" onClick={() => vlayerFlow.requestZkProof()}>
            {vlayerFlow.isZkProving ? "ZK Proofing..." : "Request ZK Proof"}
          </Button>
          {vlayerFlow.beauty && (
            <pre className="max-w-screen-lg overflow-x-auto whitespace-pre-wrap break-words">
              {(() => {
                const beautyStr = JSON.stringify(vlayerFlow.beauty);
                const firstBrace = beautyStr.indexOf("{");
                const lastBrace = beautyStr.lastIndexOf("}");
                const content = beautyStr.slice(firstBrace, lastBrace + 1);
                console.log("content", content);
                return content;
              })()}
            </pre>
          )}
        </>
      ) : null}
    </div>
  );
}

export default App;
