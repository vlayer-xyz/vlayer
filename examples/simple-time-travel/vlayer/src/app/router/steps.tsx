import {
  WelcomePage,
  ConnectWallet,
  SendEmailContainer,
  CollectEmailContainer,
  MintNFTContainer,
  SuccessContainer,
} from "../../pages";
import { Step, StepKind, stepsMeta, StepComponentMap } from "./types";


// Map step kinds to their respective components
export const stepComponents: StepComponentMap = {
  [StepKind.welcome]: WelcomePage,
};

// Get complete step data with component
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

// Get all steps as an array when needed
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
