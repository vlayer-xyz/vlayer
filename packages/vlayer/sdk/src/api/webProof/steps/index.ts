import { expectUrl, WebProofStepExpectUrl } from "./expectUrl.ts";
import { startPage, WebProofStepStartPage } from "./startPage.ts";
import { notarize, WebProofStepNotarize } from "./notarize.ts";

const steps = {
  expectUrl,
  startPage,
  notarize,
};

type WebProofStep =
  | WebProofStepNotarize
  | WebProofStepExpectUrl
  | WebProofStepStartPage;

export {
  expectUrl,
  startPage,
  notarize,
  steps,
  WebProofStep,
  WebProofStepNotarize,
  WebProofStepStartPage,
  WebProofStepExpectUrl,
};
