import DnsResolver from "dns-over-http-resolver";

export async function resolveDkimDns(domain: string, selector: string) {
  const resolver = new DnsResolver();
  const address = await resolver.resolveTxt(`${selector}._domainkey.${domain}`);

  let record = address.flat().at(-1);

  if (record?.startsWith("p=")){
    record = ["v=DKIM1", "k=rsa", record].join("; ");
  }
  
  return record;

}
