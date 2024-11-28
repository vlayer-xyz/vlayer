import { Button, Card } from "react-daisyui";
import { useVlayerFlow } from "./hooks/useProof";
import { config } from "./config";
import { useCallback } from "react";

const CheckIcon = () => (
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
);
function WebProofButton({
  vlayerFlow,
}: {
  vlayerFlow: ReturnType<typeof useVlayerFlow>;
}) {
  return (
    <Button
      loading={vlayerFlow.isWebProving}
      color="primary"
      onClick={() => {
        if (!vlayerFlow.webProof) {
          vlayerFlow.requestWebProof();
        }
      }}
    >
      {vlayerFlow.webProof ? <CheckIcon /> : null}
      {vlayerFlow.isWebProving
        ? "Web Proving..."
        : vlayerFlow.webProof
          ? "Web proof ready"
          : "Request Web Proof"}
    </Button>
  );
}

const BeautyCard = ({ beauty }: { beauty: unknown }) => {
  const formatBeautyData = useCallback((data: unknown) => {
    const beautyStr = JSON.stringify(data || {});
    const firstBrace = beautyStr.indexOf("{");
    const lastBrace = beautyStr.lastIndexOf("}");
    return beautyStr.slice(firstBrace, lastBrace + 1);
  }, []);

  if (!beauty) {
    return null;
  }

  return (
    <Card
      style={{
        backgroundColor: "#262932",
      }}
      className="text-sm p-8 text-white font-mono max-w-screen-lg overflow-x-auto whitespace-pre-wrap break-words tracking-widest"
    >
      <div className="mb-4 text-lg">Redacted Data from tink</div>
      {formatBeautyData(beauty)}
    </Card>
  );
};
function ZkProofButton({
  vlayerFlow,
}: {
  vlayerFlow: ReturnType<typeof useVlayerFlow>;
}) {
  return (
    <Button
      loading={vlayerFlow.isZkProving}
      color="primary"
      onClick={() => vlayerFlow.requestZkProof()}
      disabled={!vlayerFlow.webProof}
    >
      {vlayerFlow.zkProof ? <CheckIcon /> : null}
      {vlayerFlow.isZkProving
        ? "ZK Proofing..."
        : vlayerFlow.zkProof
          ? "Zk proof ready"
          : "Request ZK Proof"}
    </Button>
  );
}

function App() {
  const vlayerFlow = useVlayerFlow({
    webProofConfig: config,
  });

  return (
    <div className="flex flex-col items-center gap-4 mt-48">
      <WebProofButton vlayerFlow={vlayerFlow} />
      <ZkProofButton vlayerFlow={vlayerFlow} />
      <BeautyCard beauty={vlayerFlow.beauty} />
    </div>
  );
}

export default App;
