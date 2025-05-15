import { expectUrl } from "./expectUrl";
import { startPage } from "./startPage";
import { redirect } from "./redirect";
import { notarize } from "./notarize";
import { extractVariables } from "./extractVariables";

const steps = {
  expectUrl,
  redirect,
  startPage,
  notarize,
  extractVariables,
};

export { expectUrl, startPage, notarize, extractVariables, steps };
