import { LOADING, useSessionStorage } from "@vlayer/extension-hooks";
import { useCallback } from "react";
import { StorageKeys } from "src/state";

export const useStoredUserActionAssertions = () => {
  const [_assertion, _storeAssertion] = useSessionStorage<
    Record<string, boolean>
  >(StorageKeys.userActionAssertions, {});

  const assertion = _assertion === LOADING ? {} : _assertion;

  const storeAssertion = useCallback(
    (key: string, value: boolean) =>
      _storeAssertion((prev) => ({
        ...prev,
        [key]: value,
      })),
    [_storeAssertion],
  );

  return [assertion, storeAssertion] as const;
};
