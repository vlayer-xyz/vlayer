import browser from "webextension-polyfill";

export async function getElementOnPage(
  selector: string,
): Promise<Element | null> {
  const tabs = await browser.tabs.query({ active: true, currentWindow: true });
  if (!tabs || tabs.length === 0) {
    throw Error("No active tab found");
  }
  if (!tabs[0].id) {
    throw Error("Active tab has no ID");
  }
  const scriptExecutionResult = await browser.scripting.executeScript({
    target: { tabId: tabs[0].id },
    func: (selector: string) => document.querySelector(selector),
    args: [selector],
  });

  return (scriptExecutionResult?.[0]?.result as Element) ?? null;
}

export async function getActiveTabUrl() {
  const tabs = await browser.tabs.query({ active: true, currentWindow: true });
  return tabs[0]?.url;
}
