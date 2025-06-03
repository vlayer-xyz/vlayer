import { LOADING, useSessionStorage } from "@vlayer/extension-hooks";
import { useCallback } from "react";

export const useStoredUserActionAssertions = () => {
  const [_assertion, _storeAssertion] = useSessionStorage<
    Record<string, boolean>
  >("userActionAssertions", {});

  const assertion = _assertion === LOADING ? {} : _assertion;

  const storeAssertion = useCallback(
    (key: string, value: boolean) =>
      _storeAssertion({
        ...assertion,
        [key]: value,
      }),
    [assertion],
  );

  return [assertion, storeAssertion] as const;
};
