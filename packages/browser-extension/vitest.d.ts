import "vitest";
interface CustomMatchers<R = string> {
  toEqualEncodedString: (expected: EncodedString) => R;
}

declare module "vitest" {
  // eslint-disable-next-line @typescript-eslint/no-empty-object-type
  interface Assertion<T = unknown> extends CustomMatchers<T> {}
  // eslint-disable-next-line @typescript-eslint/no-empty-object-type
  interface AsymmetricMatchersContaining extends CustomMatchers {}
}
