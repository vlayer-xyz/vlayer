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
  Answer:
    | {
        name: string;
        type: number;
        TTL: number;
        data: string;
      }[]
    | undefined;
  VerificationData:
    | {
        valid_until: number;
        signature: string;
        pub_key: string;
      }
    | undefined;
}

function parseBase64(data: string): `0x${string}` {
  return `0x${Buffer.from(data, "base64").toString("hex")}`;
}

function parseVerificationData(response: DnsResponse) {
  if (!response.VerificationData) {
    console.warn(`No verification data in DNS response`);
    return {
      validUntil: 0n,
      signature: "0x" as const,
      pubKey: "0x" as const,
    };
  }
  return {
    validUntil: BigInt(response.VerificationData.valid_until),
    signature: parseBase64(response.VerificationData.signature),
    pubKey: parseBase64(response.VerificationData.pub_key),
  };
}

function takeLastAnswer(response: DnsResponse) {
  const answer = response.Answer;
  if (!answer || answer?.length == 0) {
    throw new Error(
      `No DNS answer found\n${JSON.stringify(response, null, 2)}`,
    );
  }
  const record = answer.flat().at(-1)!;
  return {
    name: record.name,
    recordType: record.type,
    data: normalizeDnsData(record.data),
    ttl: BigInt(record.TTL),
  };
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

    return {
      dnsRecord: takeLastAnswer(response),
      verificationData: parseVerificationData(response),
    };
  }
}

export function normalizeDnsData(data: string) {
  if (data.startsWith("p=")) {
    return ["v=DKIM1", "k=rsa", data].join("; ");
  }

  return data;
}
