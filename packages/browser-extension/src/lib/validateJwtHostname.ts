import { type Claims } from "./types/jwt";
import { JwtInvalidHostname } from "./errors";

export function validateJwtHostname(claims: Claims, expected: string): string {
  if (claims.host !== expected) {
    throw new JwtInvalidHostname(claims.host, expected);
  }
  return claims.host;
}
