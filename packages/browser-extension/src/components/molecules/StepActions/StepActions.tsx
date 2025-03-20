import { match, P } from "ts-pattern";
import React from "react";

import { ExpectUrlStepActions } from "./ExpectUrl";
import { NotarizeStepActions } from "./Notarize";
import { StartPageStepActions } from "./StartPage";
import { StepStatus } from "constants/step";
import {
  EXTENSION_STEP,
  ExtensionStep,
} from "src/web-proof-commons/types/message.ts";

export const StepActions: React.FC<{
  kind: ExtensionStep;
  index: number;
  link?: string;
  label: string;
  buttonText?: string;
  status: StepStatus;
}> = ({ kind, link, status, buttonText }) => {
  return (
    <>
      {match(kind)
        .with(EXTENSION_STEP.expectUrl, () => (
          <ExpectUrlStepActions status={status} />
        ))
        .with(EXTENSION_STEP.notarize, () => (
          <NotarizeStepActions
            isVisited={false}
            link={link || ""}
            buttonText={buttonText || ""}
            status={status}
          />
        ))
        .with(EXTENSION_STEP.startPage, () => (
          <StartPageStepActions
            isVisited={false}
            link={link || ""}
            buttonText={buttonText || ""}
            status={status}
          />
        ))
        .with(P.union(EXTENSION_STEP.fetchAndNotarize,EXTENSION_STEP.extractVariables), () => {
          console.warn("Unsupported step type:", kind);
          return <></>;
        })
        .exhaustive()}
    </>
  );
};
