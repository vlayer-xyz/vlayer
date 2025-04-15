import {
  type DkimDomainSelector,
  getDkimSigners,
  parseEmail,
} from "./parseEmail";
import { DnsResolver } from "./dnsResolver";
import { prefixAllButNthSubstring } from "../utils/prefixAllButNthSubstring";
import {
  HttpAuthorizationError,
  httpAuthorizationErrorWithNote,
} from "../lib/errors";
import { match, P } from "ts-pattern";

export function findIndicesOfMatchingDomains(
  signers: DkimDomainSelector[],
  expectedOrigin: string,
) {
  return signers
    .map(({ domain }) => expectedOrigin.endsWith(`@${domain}`))
    .map((isMatch, index) => (isMatch ? index : -1))
    .filter((index) => index !== -1);
}

function requireSameOrigin(
  mimeEmail: string,
  signers: DkimDomainSelector[],
  fromAddress: string,
) {
  const matchingIndices = findIndicesOfMatchingDomains(signers, fromAddress);

  if (matchingIndices.length != 1) {
    throw new Error(
      `Found ${matchingIndices.length} DKIM headers matching the sender domain`,
    );
  }

  const [matchingIndex] = matchingIndices;

  return [
    prefixAllButNthSubstring(
      mimeEmail,
      /^\s*dkim-signature/gim,
      signers.length,
      matchingIndex,
    ),
    [signers[matchingIndex]] as DkimDomainSelector[],
  ] as const;
}

export async function preverifyEmail({
  mimeEmail,
  dnsResolverUrl,
  token,
}: {
  mimeEmail: string;
  dnsResolverUrl: string;
  token?: string;
}) {
  const parsedEmail = await parseEmail(mimeEmail);
  let signers = getDkimSigners(parsedEmail);
  const fromAddress = parsedEmail.from.address;

  if (!fromAddress) {
    throw new Error("No from address found");
  }
  if (signers.length === 0) {
    throw new Error("No DKIM header found");
  }
  [mimeEmail, signers] = requireSameOrigin(mimeEmail, signers, fromAddress);

  const [{ domain, selector }] = signers;
  const resolver = new DnsResolver(dnsResolverUrl, token);

  try {
    const dnsResponse = await resolver.resolveDkimDns(selector, domain);

    return {
      email: mimeEmail,
      ...dnsResponse,
    };
  } catch (error) {
    const errorWithNote = match(error)
      .with(P.instanceOf(HttpAuthorizationError), (error) =>
        httpAuthorizationErrorWithNote(error),
      )
      .otherwise((error) => error);
    throw errorWithNote;
  }
}
