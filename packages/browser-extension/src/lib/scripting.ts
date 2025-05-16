export async function getElementOnPage(
  selector: string,
): Promise<string | null> {
  return new Promise((resolve, revert) =>
    chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
      chrome.scripting
        .executeScript({
          target: { tabId: tabs[0].id! },
          func: (selector) => document.querySelector(selector)?.textContent,
          args: [selector],
        })
        .then((queryResult) => resolve(queryResult?.[0]?.result ?? null))
        .catch(revert);
    }),
  );
}
