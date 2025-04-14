import { type Overrides, type DefinedOverrides } from "./types";

export function filterOverrides(overrides: Overrides): DefinedOverrides {
  const defined: DefinedOverrides = {};
  for (const key in overrides) {
    const value = overrides[key as keyof Overrides];
    if (value !== undefined) {
      defined[key] = value;
    }
  }
  return defined;
}
