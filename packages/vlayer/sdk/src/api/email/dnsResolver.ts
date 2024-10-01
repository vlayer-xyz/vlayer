import DnsResolver from "dns-over-http-resolver";

export async function resolveDkimDns(domain: string, selector: string) {
  const resolver = new DnsResolver();
  const address = await resolver.resolveTxt(`${selector}._domainkey.${domain}`);
  return address.flat()[0];
}
