import { expectUrl } from "./expectUrl";
import { startPage } from "./startPage";
import { notarize } from "./notarize";
import { notarizeGql } from "./notarizeGql";
const steps = {
  expectUrl,
  startPage,
  notarize,
  notarizeGql,
};

export { expectUrl, startPage, notarize, steps, notarizeGql };
