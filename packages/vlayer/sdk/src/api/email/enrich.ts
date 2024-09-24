import { parseEmail, getDkimSigners } from "./parseEmail.ts";
import { resolveDkimDns } from "./dnsResolver.ts";

export async function enrichEmail(mimeEmail: string) {
  const parsedEmail = await parseEmail(mimeEmail);
  const [{ domain, selector }] = getDkimSigners(parsedEmail);
  const dnsRecord = await resolveDkimDns(domain, selector);
  return {
    email: mimeEmail,
    dnsRecord,
  };
}
