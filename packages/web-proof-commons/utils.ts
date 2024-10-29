import { Address, isAddress } from "viem";

declare const __brand: unique symbol;
type Brand<B> = { [__brand]: B };
export type Branded<T, B> = T & Brand<B>;

export function isDefined<T>(
  value: T | undefined,
  message: string = "Value is undefined",
): asserts value is T {
  if (value === undefined) {
    throw new Error(message);
  }
}

export function handleAsyncError<T>(
  fn: (...args: unknown[]) => Promise<T>,
  message: string = "Error during async call: ",
): () => void {
  return () => {
    fn().catch((error) => {
      console.error(message, error);
    });
  };
}

export function asAddress(value: unknown): Address {
  if (typeof value === "string" && isAddress(value)) {
    return value;
  } else {
    throw new Error(`Invalid address: ${String(value)}`);
  }
}
