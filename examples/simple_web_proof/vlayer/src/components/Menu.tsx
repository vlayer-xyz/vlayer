import { useState, useEffect } from "react";
import { useVlayerFlow } from "../hooks/useProof";

function Menu({
  vlayerFlow,
}: {
  vlayerFlow: ReturnType<typeof useVlayerFlow>;
}) {
  const [isLoading, setIsLoading] = useState(false);

  const handleRequestProve = async () => {
    try {
      setIsLoading(true);
      vlayerFlow.requestWebProof();
    } catch (error) {
      console.error(error);
    }
  };

  const currentStep = () => {
    switch (vlayerFlow.stage) {
      case "web_proof_requested":
        return "Generating Web Proof (1/3)";
      case "zk_proof_requested":
        return "Generating ZK Proof (2/3)";
      case "verification_requested":
        return "Verifying on-chain (3/3)";
      case vlayerFlow.completed:
        return "🎉 Proof verified successfully!";
      default:
        return "Generate Proof of Twitter Handle";
    }
  };

  useEffect(() => {
    if (vlayerFlow.webProof) {
      vlayerFlow.requestZkProof();
    }
  }, [vlayerFlow.webProof]);

  useEffect(() => {
    if (vlayerFlow.zkProof) {
      vlayerFlow.requestVerification();
    }
  }, [vlayerFlow.zkProof]);

  useEffect(() => {
    if (vlayerFlow.completed) {
      setIsLoading(false);
    }
  }, [vlayerFlow.completed]);

  useEffect(() => {
    if (vlayerFlow.isError) {
      throw new Error("Check console for details");
    }
  }, [vlayerFlow.isError]);

  return (
    <div className="min-h-screen flex flex-col items-center justify-center p-4">
      <div className="card w-96 shadow-xl bg-violet-100">
        <figure className="px-10 pt-10">
          <img
            src="vlayer_logo.svg"
            alt="Logo"
            className="rounded-xl object-cover"
          />
        </figure>
        <div className="card-body items-center text-center space-y-4">
          <div className="btn-group-vertical w-full">
            <button
              className="btn btn-primary w-full mb-2 text-white"
              onClick={handleRequestProve}
            >
              <>
                {isLoading && <span className="loading loading-spinner"></span>}
                {currentStep()}
              </>
            </button>
          </div>
          {vlayerFlow.completed && (
            <div className="text-block w-full mt-5">
              Verification hash: <br />
              {`${vlayerFlow.verification.transactionHash.slice(0, 6)}...${vlayerFlow.verification.transactionHash.slice(-4)}`}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default Menu;
