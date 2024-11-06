import { useBrowsingHistory } from "./useBrowsingHistory";
import { useProvingSessionConfig } from "./useProvingSessionConfig";
import { HistoryItem } from "../state/history";
import urlPattern from "url-pattern";
export function useProvenUrl(): HistoryItem | undefined {
  const [config] = useProvingSessionConfig();
  const [browsingHistory] = useBrowsingHistory();

  const steps = config?.steps || [];
  const step = steps.find(
    ({ step }) => step === "notarize" || step === "notarizeGql",
  );

  if (!step) {
    return undefined;
  }

  return (
    browsingHistory.find((item) => item.url.includes(step.url)) ||
    browsingHistory.find((item: HistoryItem) => {
      try {
        console.log("item");
        const stepUrl = new URL(step.url);
        const itemUrl = new URL(item.url);
        const pattern = new urlPattern(stepUrl.pathname);

        // eslint-disable-next-line @typescript-eslint/no-unsafe-assignment
        const matchesQuery = pattern.match(itemUrl.pathname);

        if (stepUrl.origin === itemUrl.origin && matchesQuery) {
          console.log("matchesQuery", step);
          const gqlParams = itemUrl.searchParams;
          //@ts-expect-error ddd
          const gqlQueryMatch = Object.keys(step.query).every((query) => {
            //@ts-expect-error derwer
            const actualValue = JSON.parse(
              decodeURIComponent(gqlParams.get(query)),
            );
            console.log("actualValue", actualValue);
            console.log("step.query[query]", step.query[query]);
            return (
              JSON.stringify(actualValue) === JSON.stringify(step.query[query])
            );
          });
          console.log("gqlQueryMatch", gqlQueryMatch);

          return true;
        }
      } catch (e) {
        console.log("error", e);
      }
    })
  );
}
