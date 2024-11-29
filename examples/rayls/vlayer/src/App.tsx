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
      color={vlayerFlow.webProof ? "success" : "primary"}
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

const BeautyCard = ({
  beauty,
  zkProof,
}: {
  beauty: unknown;
  zkProof: unknown[];
}) => {
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
    <div className="w-full flex justify-center">
      <Card
        style={{
          backgroundColor: "#262932",
          width: "60vw",
        }}
        className="text-sm p-8 text-white font-mono overflow-x-auto whitespace-pre-wrap break-words tracking-widest"
      >
        {beauty && (
          <>
            <div className="mb-4 text-lg">Redacted Data from tink</div>
            {formatBeautyData(beauty)}
          </>
        )}

        {zkProof && (
          <>
            <div className="mb-4 text-lg mt-4">Zk public output</div>
            <div>{JSON.stringify(zkProof[1])}</div>
          </>
        )}
      </Card>
    </div>
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
      color={vlayerFlow.zkProof ? "success" : "primary"}
      onClick={() => vlayerFlow.requestZkProof()}
      disabled={!vlayerFlow.webProof}
    >
      {vlayerFlow.zkProof ? <CheckIcon /> : null}
      {vlayerFlow.isZkProving
        ? "ZK Proving..."
        : vlayerFlow.zkProof
          ? "Zk proof ready"
          : "Request ZK Proof"}
    </Button>
  );
}
function VerificationButton({
  vlayerFlow,
}: {
  vlayerFlow: ReturnType<typeof useVlayerFlow>;
}) {
  return (
    <Button
      onClick={() => vlayerFlow.requestVerification()}
      disabled={!vlayerFlow.zkProof}
      color={vlayerFlow.verification ? "success" : "primary"}
    >
      Verify
    </Button>
  );
}

function App() {
  const vlayerFlow = useVlayerFlow({
    webProofConfig: config,
  });

  return (
    <div className="flex flex-col items-center gap-4 mt-48">
      <div className="flex flex-col gap-4">
        <BeautyCard
          beauty={vlayerFlow.beauty || ""}
          zkProof={vlayerFlow.zkProof as unknown[]}
        />
        <div className="flex gap-4" style={{ width: "60vw" }}>
          <WebProofButton vlayerFlow={vlayerFlow} />
          <ZkProofButton vlayerFlow={vlayerFlow} />
          <VerificationButton vlayerFlow={vlayerFlow} />
        </div>
      </div>
    </div>
  );
}

export default App;
