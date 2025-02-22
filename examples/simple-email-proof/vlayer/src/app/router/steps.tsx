import {
  WelcomePage,
  ConnectWallet,
  SendEmailContainer,
  CollectEmailContainer,
  MintNFTContainer,
  SuccessContainer,
} from "../../pages";
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
  CONNECT_WALLET,
  SEND_EMAIL,
  COLLECT_EMAIL,
  MINT_NFT,
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
  {
    path: "connect-wallet",
    kind: STEP_KIND.CONNECT_WALLET,
    component: ConnectWallet,
    title: "Mail based NFT",
    description:
      "To proceed to the next step, please connect your wallet now by clicking the button below.",
    backUrl: "",
    index: 1,
  },
  {
    path: "send-email",
    kind: STEP_KIND.SEND_EMAIL,
    component: SendEmailContainer,
    title: "Send Email",
    description:
      "Please copy the details provided below and use them to send the email.",
    backUrl: "connect-wallet",
    index: 2,
  },
  {
    path: "collect-email",
    kind: STEP_KIND.COLLECT_EMAIL,
    component: CollectEmailContainer,
    title: "Waiting...",
    description:
      "Our mailbox is processing your email. Please wait a few seconds.",
    backUrl: "send-email",
    index: 2,
  },
  {
    path: "mint-nft",
    kind: STEP_KIND.MINT_NFT,
    component: MintNFTContainer,
    title: "Mint NFT",
    description: "Your email is ready for proving and minting.",
    backUrl: "send-email",
    index: 3,
  },
  {
    path: "success",
    headerIcon: "/img/success-icon.svg",
    kind: STEP_KIND.SUCCESS,
    component: SuccessContainer,
    title: "Success",
    description: "",
    index: 4,
  },
];

export class StepNotFoundError extends Error {
  constructor(kind: STEP_KIND) {
    super(`Step with kind ${kind} not found`);
    this.name = "StepNotFoundError";
  }
}

export const getStepPath = (kind: STEP_KIND): string => {
  const step = steps.find((step) => step.kind === kind);
  if (!step) {
    throw new StepNotFoundError(kind);
  }
  return step.path;
};

export const getStepBackUrl = (kind: STEP_KIND): string => {
  const step = steps.find((step) => step.kind === kind);
  if (!step) {
    throw new StepNotFoundError(kind);
  }
  return step.backUrl || "";
};
