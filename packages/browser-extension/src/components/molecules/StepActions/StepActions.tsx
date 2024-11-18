import { match } from "ts-pattern";
import React from "react";

import { ExpectUrlStepActions } from "./ExpectUrl";
import { NotarizeStepActions } from "./Notarize";
import { StartPageStepActions } from "./StartPage";
import { StepStatus } from "constants/step";

export const StepActions: React.FC<{
  kind: "notarize" | "expectUrl" | "startPage";
  index: number;
  link?: string;
  label: string;
  buttonText?: string;
  status: StepStatus;
}> = ({ kind, link, status, buttonText }) => {
  return (
    <>
      {match(kind)
        .with("expectUrl", () => <ExpectUrlStepActions status={status} />)
        .with("notarize", () => (
          <NotarizeStepActions
            isVisited={false}
            link={link || ""}
            buttonText={buttonText || ""}
            status={status}
          />
        ))
        .with("startPage", () => (
          <StartPageStepActions
            isVisited={false}
            link={link || ""}
            buttonText={buttonText || ""}
            status={status}
          />
        ))
        .exhaustive()}
    </>
  );
};
