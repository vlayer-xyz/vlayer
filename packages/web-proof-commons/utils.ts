type Brand<B> = { __brand: B };
export type Branded<T, B> = T & Brand<B>;

export function isDefined<T>(
  value: T | undefined,
  message: string = "Value is undefined",
): asserts value is T {
  if (value === undefined) {
    throw new Error(message);
  }
}
