type CamelCase<T extends string> = T extends `${infer F}_${infer R}`
  ? `${Lowercase<F>}${Capitalize<CamelCase<R>>}`
  : Lowercase<T>;

type CamelCasedKeys<T extends Record<string, unknown>> = {
  [K in keyof T as CamelCase<K extends string ? K : never>]: T[K];
};

export const keysToCamelCase = <T extends Record<string, unknown>>(
  obj: T,
): CamelCasedKeys<T> => {
  return Object.fromEntries(
    Object.entries(obj).map(([key, value]) => [toCamelCase(key), value]),
  ) as CamelCasedKeys<T>;
};

export const toCamelCase = <T extends string>(str: T): CamelCase<T> =>
  str
    .toLowerCase()
    .replace(
      /([-_]+[a-z])/g,
      (group) => group.at(-1)?.toUpperCase() ?? "",
    ) as CamelCase<T>;
