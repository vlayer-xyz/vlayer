import { expectUrl } from "./expectUrl";
import { startPage } from "./startPage";
import { redirect } from "./redirect";
import { notarize } from "./notarize";
import { extractVariables } from "./extractVariables";
import { userAction } from "./userAction";

const steps = {
  expectUrl,
  redirect,
  startPage,
  notarize,
  extractVariables,
  userAction,
};

export { expectUrl, startPage, notarize, extractVariables, userAction, steps };
