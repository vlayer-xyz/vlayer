import { Button, Card } from "react-daisyui";
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
        loading={vlayerFlow.isWebProving}
        color="primary"
        onClick={() => {
          if (!vlayerFlow.webProof) {
            vlayerFlow.requestWebProof();
          }
        }}
      >
        {vlayerFlow.webProof ? (
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="h-6 w-6 mr-2"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M5 13l4 4L19 7"
            />
          </svg>
        ) : (
          ""
        )}
        {vlayerFlow.isWebProving
          ? "Web Proving..."
          : vlayerFlow.webProof
            ? "Web proof ready"
            : "Request Web Proof"}
      </Button>

      {vlayerFlow.webProof ? (
        <>
          <Button
            loading={vlayerFlow.isZkProving}
            color="primary"
            onClick={() => vlayerFlow.requestZkProof()}
          >
            {vlayerFlow.zkProof ? (
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="h-6 w-6 mr-2"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            ) : (
              ""
            )}
            {vlayerFlow.isZkProving
              ? "ZK Proofing..."
              : vlayerFlow.zkProof
                ? "Zk proof ready"
                : "Request ZK Proof"}
          </Button>
          {vlayerFlow.beauty && (
            <Card
              style={{
                backgroundColor: "#262932",
              }}
              className="text-sm p-8 text-white font-mono max-w-screen-lg overflow-x-auto whitespace-pre-wrap break-words tracking-widest"
            >
              <div className="mb-4 text-lg ">Redacted Data from tink </div>
              {(() => {
                const beautyStr = JSON.stringify(vlayerFlow.beauty);
                const firstBrace = beautyStr.indexOf("{");
                const lastBrace = beautyStr.lastIndexOf("}");
                const content = beautyStr.slice(firstBrace, lastBrace + 1);
                return content;
              })()}
            </Card>
          )}
        </>
      ) : null}
    </div>
  );
}

export default App;
