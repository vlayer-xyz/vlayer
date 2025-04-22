import { type Claims } from "./types/jwt";
import { JwtInvalidHostname, JwtMissingHostname } from "./errors";

export function validateJwtHostname(claims: Claims, expected: string): string {
  if (claims.host === undefined) {
    throw new JwtMissingHostname(expected);
  }

  if (claims.host !== expected) {
    throw new JwtInvalidHostname(claims.host, expected);
  }

  return claims.host;
}
