import { expectUrl } from "./expectUrl";
import { startPage } from "./startPage";
import { redirect } from "./redirect";
import { notarize } from "./notarize";

const steps = {
  expectUrl,
  redirect,
  startPage,
  notarize,
};

export { expectUrl, startPage, notarize, steps };
