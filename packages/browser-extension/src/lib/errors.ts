export class JwtInvalidHostname extends Error {
  constructor(given: string, expected: string) {
    super(
      `Invalid JWT hostname: JWT valid for hostname ${given}, but needs ${expected}`,
    );
    this.name = "JwtInvalidHostname";
  }
}

export class JwtMissingHostname extends Error {
  constructor(expected: string) {
    super(
      `Missing JWT hostname: no hostname given, but Web Proof needs ${expected}`,
    );
    this.name = "JwtMissingHostname";
  }
}
