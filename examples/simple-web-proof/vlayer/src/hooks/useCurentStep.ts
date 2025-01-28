import { useEffect, useState } from "react";
import { useLocation } from "react-router";

type Step = {
  name: string;
  url: string;
  backUrl?: string;
};

const STEPS: Step[] = [
  {
    name: "Welcome",
    url: "/",
  },
  {
    name: "Connect Wallet",
    url: "/connect-wallet",
  },
  {
    name: "Get data from X",
    url: "/start-proving",
    backUrl: "/connect-wallet",
  },
  {
    name: "Mint NFT",
    url: "/minting",
    backUrl: "/start-proving",
  },
  {
    name: "Success",
    url: "/success",
  },
];

export const useCurrentStep = () => {
  console.log("useCurrentStep");
  const location = useLocation();
  console.log(location.pathname);
  const [currentStep, setCurrentStep] = useState<
    (Step & { index: number }) | undefined
  >(undefined);

  useEffect(() => {
    console.log("useEffect");
    console.log(location.pathname.split("/")[2]);
    setCurrentStep(
      STEPS.map((step, index) => ({ ...step, index })).find(
        (step) => step.url === `/${location.pathname.split("/")[2]}`,
      ),
    );
  }, [location.pathname]);
  return { currentStep };
};
