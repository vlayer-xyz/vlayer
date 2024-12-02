import { parseEmail, getDkimSigners } from "./parseEmail";
import { resolveDkimDns } from "./dnsResolver";
import { prefixAllButNthSubstring } from "../utils/prefixAllButNthSubstring";

function requireSameOrigin(
  mimeEmail: string,
  signers: { domain: string; selector: string }[],
  fromAddress: string,
) {
  const matchingIndices = signers
    .map(({ domain }) => fromAddress.endsWith(domain))
    .map((isMatch, index) => (isMatch ? index : -1))
    .filter((index) => index !== -1);

  if (matchingIndices.length != 1) {
    throw new Error(
      `Found ${matchingIndices.length} DKIM headers matching the sender domain`,
    );
  }

  const [matchingIndex] = matchingIndices;

  return [
    prefixAllButNthSubstring(mimeEmail, "dkim-signature", matchingIndex),
    [signers[matchingIndex]] as any[],
  ] as const;
}

export async function preverifyEmail(mimeEmail: string) {
  const parsedEmail = await parseEmail(mimeEmail);
  let signers = getDkimSigners(parsedEmail);
  const fromAddress = parsedEmail.from.address;

  if (!fromAddress) {
    throw new Error("No from address found");
  }
  if (signers.length === 0) {
    throw new Error("No DKIM header found");
  }
  if (signers.length > 1) {
    [mimeEmail, signers] = requireSameOrigin(mimeEmail, signers, fromAddress);
  }

  const [{ domain, selector }] = signers;
  const dnsRecord = await resolveDkimDns(domain, selector);
  return {
    email: mimeEmail,
    dnsRecords: [dnsRecord],
  };
}
