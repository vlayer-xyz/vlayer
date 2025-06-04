import type { WebProofStepUserAction } from "../../web-proof-commons";
import { getActiveTabUrl, getElementOnPage } from "lib/activeTabContext.ts";
import { URLPattern } from "urlpattern-polyfill";

const isActiveTabUrlMatching = async (expectedUrl: string) => {
  const currentUrl = await getActiveTabUrl();
  if (!currentUrl) {
    return false;
  }
  return new URLPattern(expectedUrl).test(currentUrl);
};

export const domStateAssertion = (
  element: Element | null,
  assertion: WebProofStepUserAction["assertion"],
) => {
  if (element === null) {
    return assertion.require.notExist;
  }
  return assertion.require.exist;
};

export const isExpectedDomElementState = async (
  step: WebProofStepUserAction,
  storedAssertion: boolean | undefined,
  storeAssertion: (value: boolean) => void,
) => {
  if (!(await isActiveTabUrlMatching(step.url))) {
    return Boolean(storedAssertion);
  }

  let element: Element | null;
  try {
    element = await getElementOnPage(step.assertion.domElement);
  } catch (e) {
    console.error(
      `Error getting element ${step.assertion.domElement} on page:`,
      e,
    );
    return false;
  }

  const result = domStateAssertion(element, step.assertion);
  storeAssertion(result);

  return result;
};
