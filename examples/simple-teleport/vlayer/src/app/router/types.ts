import { ComponentType } from "react";

export enum StepKind {
  welcome,
  showBalance,
  confirmMint,
  success,
}

export type StepMeta = {
  path: string;
  kind: StepKind;
  title: string;
  description: string;
  headerIcon?: string;
  index: number;
  backUrl?: string;
};

export const stepsMeta: Record<StepKind, StepMeta> = {
  [StepKind.welcome]: {
    path: "",
    kind: StepKind.welcome,
    title: "Superchain Whale NFT",
    description: "Prove that you own USDC token across multiple chains.",
    headerIcon: "/img/teleport-icon.svg",
    index: 0,
  },
  [StepKind.showBalance]: {
    path: "show-balance",
    kind: StepKind.showBalance,
    title: "Superchain Whale NFT",
    description: "Prove that you own USDC token across multiple chains.",
    headerIcon: "/img/teleport-icon.svg",
    index: 1,
  },
  [StepKind.confirmMint]: {
    path: "confirm-mint",
    kind: StepKind.confirmMint,
    title: "Mint NFT Confirmation",
    description: "",
    headerIcon: "/img/teleport-icon.svg",
    index: 2,
  },
  [StepKind.success]: {
    path: "success",
    kind: StepKind.success,
    title: "Success",
    description: "",
    headerIcon: "/img/success.svg",
    index: 3,
  },
};

export type StepComponentMap = Record<StepKind, ComponentType>;

export type Step = StepMeta & {
  component: ComponentType;
};
