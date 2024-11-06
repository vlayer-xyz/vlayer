import { useLocalStorage, LOADING } from "@vlayer/extension-hooks";
import { ZkProvingStatus } from "../web-proof-commons";

export function isValidZkProvingStatus(
  value: unknown,
): value is ZkProvingStatus {
  return (value as string) in ZkProvingStatus;
}

export class InvalidZkProvingStatus extends Error {
  constructor(value: string) {
    super(`Invalid zk proving status ${value}`);
  }
}

export const useZkProvingState = (): {
  value: ZkProvingStatus;
  isError: boolean;
  error: Error | undefined;
  isProving: boolean;
} => {
  const [state] = useLocalStorage<ZkProvingStatus>(
    "zkProvingStatus",
    ZkProvingStatus.NotStarted,
  );

  return {
    value: state === LOADING ? ZkProvingStatus.NotStarted : state,
    isError: state !== LOADING && !isValidZkProvingStatus(state),
    error:
      state !== LOADING && !isValidZkProvingStatus(state)
        ? new InvalidZkProvingStatus(state)
        : undefined,
    isProving: state === ZkProvingStatus.Proving,
  };
};
