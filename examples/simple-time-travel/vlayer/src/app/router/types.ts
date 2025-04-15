import { ComponentType } from "react";

export enum StepKind {
  welcome,
  showBalance,
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
    title: "Hodler NFT",
    description: "Prove that you HODL a token for at least a year.",
    headerIcon: "/img/time-travel-icon.svg",
    index: 0,
  },
  [StepKind.showBalance]: {
    path: "show-balance",
    kind: StepKind.showBalance,
    title: "Hodler NFT",
    description: "Prove that you HODL a token for at least a year.",
    headerIcon: "/img/time-travel-icon.svg",
    index: 1,
  },
  [StepKind.success]: {
    path: "success",
    kind: StepKind.success,
    title: "Success",
    description: "",
    headerIcon: "/img/success.svg",
    index: 2,
  },
};

export type StepComponentMap = Record<StepKind, ComponentType>;

export type Step = StepMeta & {
  component: ComponentType;
};
