export async function getElementOnPage(
  selector: string,
): Promise<string | null> {
  return new Promise((resolve, reject) =>
    chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
      if (!tabs || tabs.length === 0) {
        return reject(new Error("No active tab found"));
      }
      if (!tabs[0].id) {
        return reject(new Error("Active tab has no ID"));
      }
      chrome.scripting
        .executeScript({
          target: { tabId: tabs[0].id },
          func: (selector) => document.querySelector(selector)?.textContent,
          args: [selector],
        })
        .then((queryResult) => resolve(queryResult?.[0]?.result ?? null))
        .catch(reject);
    }),
  );
}
