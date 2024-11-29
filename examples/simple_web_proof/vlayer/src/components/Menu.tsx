import { useState } from "react";

function Menu({
  requestProve,
  callProver,
  callVerifier,
}: {
  requestProve: () => Promise<void>;
  callProver: () => Promise<void>;
  callVerifier: () => Promise<void>;
}) {
  const [currentStep, setCurrentStep] = useState<string>("Generating Web Proof...");
  const [isLoading, setIsLoading] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);

  const handleRequestProve = async () => {
    try {
      setIsSuccess(false);
      setIsLoading(true);
      await requestProve();
      setCurrentStep("Generating ZK Proof...");
      await callProver();
      setCurrentStep("Verifying on-chain...");
      await callVerifier();
      setIsLoading(false);
      setIsSuccess(true);
    } catch (error) {
      console.error(error);
    } finally {
      setIsLoading(false);
    }
  };

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
              {isLoading ? (
                <>
                  <span className="loading loading-spinner"></span>
                  {currentStep}
                </>
              ) : (
                "Generate Proof of Twitter"
              )}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Menu;
