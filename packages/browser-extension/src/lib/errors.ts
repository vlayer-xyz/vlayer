export class JwtInvalidHostname extends Error {
  constructor(given: string, expected: string) {
    super(
      `Invalid JWT hostname: JWT valid for hostname ${given}, but needs ${expected}`,
    );
    this.name = "JwtInvalidHostname";
  }
}
