import { Step, StepKind, stepsMeta, StepComponentMap } from "./types";
import {
  WelcomePage,
  ShowBalancePage,
  SuccessPage,
  ConfirmMintPage,
} from "../../pages";

export const stepComponents: StepComponentMap = {
  [StepKind.welcome]: WelcomePage,
  [StepKind.showBalance]: ShowBalancePage,
  [StepKind.confirmMint]: ConfirmMintPage,
  [StepKind.success]: SuccessPage,
};

export const getStep = (kind: StepKind): Step => {
  const meta = stepsMeta[kind];
  if (!meta) {
    throw new StepNotFoundError(kind);
  }

  return {
    ...meta,
    kind,
    component: stepComponents[kind],
  };
};

export const getAllSteps = (): Step[] => {
  return Object.entries(stepsMeta).map(([kindStr, meta]) => {
    const kind = Number(kindStr) as StepKind;
    return {
      ...meta,
      kind,
      component: stepComponents[kind],
    };
  });
};

export class StepNotFoundError extends Error {
  constructor(kind: StepKind) {
    super(`Step with kind ${kind} not found`);
    this.name = "StepNotFoundError";
  }
}

export const getStepPath = (kind: StepKind): string => {
  const meta = stepsMeta[kind];
  if (!meta) {
    throw new StepNotFoundError(kind);
  }
  return meta.path;
};
