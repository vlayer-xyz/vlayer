import { expectUrl } from "./expectUrl";
import { startPage } from "./startPage";
import { redirect } from "./redirect";
import { notarize } from "./notarize";
import { userAction } from "./userAction";

const steps = {
  expectUrl,
  redirect,
  startPage,
  notarize,
  userAction,
};

export { expectUrl, startPage, notarize, userAction, steps };
