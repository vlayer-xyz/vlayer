import { match, P } from "ts-pattern";
import React from "react";

import { ExpectUrlStepActions } from "./ExpectUrl";
import { NotarizeStepActions } from "./Notarize";
import { StartPageStepActions } from "./StartPage";
import { RedirectStepActions } from "./Redirect";
import { UserActionStepActions } from "./UserAction";
import { EXTENSION_STEP } from "src/web-proof-commons";
import { StepProps } from "../Step";

export const StepActions: React.FC<StepProps> = ({
  kind,
  link,
  status,
  step,
}) => {
  return (
    <>
      {match(step)
        .with({ step: EXTENSION_STEP.expectUrl }, () => (
          <ExpectUrlStepActions status={status} />
        ))
        .with({ step: EXTENSION_STEP.notarize }, () => (
          <NotarizeStepActions
            isVisited={false}
            link={link || ""}
            buttonText={""}
            status={status}
          />
        ))
        .with({ step: EXTENSION_STEP.startPage }, () => (
          <StartPageStepActions
            isVisited={false}
            link={link || ""}
            buttonText={""}
            status={status}
          />
        ))
        .with({ step: EXTENSION_STEP.redirect }, () => (
          <RedirectStepActions
            isVisited={false}
            link={link || ""}
            buttonText={""}
            status={status}
          />
        ))
        .with({ step: EXTENSION_STEP.userAction }, (step) => (
          <UserActionStepActions
            status={status}
            instruction={step.instruction}
          />
        ))
        .with(
          {
            step: P.union(
              EXTENSION_STEP.extractVariables,
              EXTENSION_STEP.clickButton,
            ),
          },
          () => {
            console.warn("Unsupported step type:", kind);
            return <></>;
          },
        )
        .exhaustive()}
    </>
  );
};
