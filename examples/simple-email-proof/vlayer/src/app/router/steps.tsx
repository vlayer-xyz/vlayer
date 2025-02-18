import WelcomePage from "../../pages/welcome";
import React from "react";

export type Step = {
  kind: STEP_KIND;
  path: string;
  backUrl?: string;
  component: React.ComponentType;
  title: string;
  description: string;
  headerIcon?: string;
  index: number;
};

export enum STEP_KIND {
  WELCOME,
  SUCCESS,
}
export const steps: Step[] = [
  {
    path: "",
    kind: STEP_KIND.WELCOME,
    component: WelcomePage,
    title: "Domain NFT",
    description:
      "Mint an NFT with your email domain. For example, if you work at Ethereum Foundation, you can mint “ethereum.org” NFT. This showcases Email Proofs.",
    headerIcon: "/img/email-welcome-img.svg",
    index: 0,
  },
];
