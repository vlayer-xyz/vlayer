import {
  WelcomePage,
  ConnectWallet,
  SendEmailContainer,
  CollectEmailContainer,
  MintNFTContainer,
  SuccessContainer,
} from "../../pages";
import React from "react";
import { STEP_KIND, stepsMeta, StepMeta } from "./types";

// Define a component mapping type
export type StepComponentMap = Record<STEP_KIND, React.ComponentType>;

// Map step kinds to their respective components
export const stepComponents: StepComponentMap = {
  [STEP_KIND.WELCOME]: WelcomePage,
  [STEP_KIND.CONNECT_WALLET]: ConnectWallet,
  [STEP_KIND.SEND_EMAIL]: SendEmailContainer,
  [STEP_KIND.COLLECT_EMAIL]: CollectEmailContainer,
  [STEP_KIND.MINT_NFT]: MintNFTContainer,
  [STEP_KIND.SUCCESS]: SuccessContainer,
};

// Create a complete step structure that combines metadata with components
export type Step = StepMeta & {
  component: React.ComponentType;
};

// Get complete step data with component
export const getStep = (kind: STEP_KIND): Step => {
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
    const kind = Number(kindStr) as STEP_KIND;
    return {
      ...meta,
      kind,
      component: stepComponents[kind],
    };
  });
};

export class StepNotFoundError extends Error {
  constructor(kind: STEP_KIND) {
    super(`Step with kind ${kind} not found`);
    this.name = "StepNotFoundError";
  }
}

export const getStepPath = (kind: STEP_KIND): string => {
  const meta = stepsMeta[kind];
  if (!meta) {
    throw new StepNotFoundError(kind);
  }
  return meta.path;
};
