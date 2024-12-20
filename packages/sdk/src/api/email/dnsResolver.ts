interface DnsResponse {
  Status: number;
  TC: boolean;
  RD: boolean;
  RA: boolean;
  AD: boolean;
  CD: boolean;
  Question: {
    name: string;
    type: number;
  }[];
  Answer: {
    name: string;
    type: number;
    TTL: number;
    data: string;
  }[];
}

export class DnsResolver {
  constructor(private host = "https://dns.google/resolve") {}

  async resolveDkimDns(selector: string, domain: string) {
    const response = (await (
      await fetch(
        `${this.host}?name=${selector}._domainkey.${domain}&type=TXT`,
        {
          headers: {
            accept: "application/dns-json",
          },
        },
      )
    ).json()) as DnsResponse;

    return response.Answer.map((answer) => answer.data);
  }
}

export async function resolveDkimDns(
  resolver: DnsResolver,
  domain: string,
  selector: string,
) {
  const address = await resolver.resolveDkimDns(selector, domain);

  let record = address.flat().at(-1);

  if (!record) {
    throw new Error("No DKIM DNS record found");
  }

  if (record?.startsWith("p=")) {
    record = ["v=DKIM1", "k=rsa", record].join("; ");
  }

  return record;
}
