import { parseEmail, getDkimSigners } from "./parseEmail.ts";
import { resolveDkimDns } from "./dnsResolver.ts";

export async function preverifyEmail(mimeEmail: string) {
  const parsedEmail = await parseEmail(mimeEmail);
  const signers = getDkimSigners(parsedEmail);
  if (signers.length === 0) {
    throw new Error("No DKIM header found");
  }
  if (signers.length > 1) {
    throw new Error("Multiple DKIM headers found");
  }
  const [{ domain, selector }] = signers;
  const dnsRecord = await resolveDkimDns(domain, selector);
  return {
    email: mimeEmail,
    dnsRecords: [dnsRecord],
  };
}
