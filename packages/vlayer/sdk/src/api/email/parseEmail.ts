import PostalMime, { Header, Email } from "postal-mime";

export class DkimParsingError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "DkimParsingError";
  }
}

export async function parseEmail(mime: string) {
  const email = await PostalMime.parse(mime.trim());

  return {
    ...email,
    parsedHeaders: email.headers.reduce((previous, current) => {
      const existing = previous.get(current.key);

      const newValue = {
        params: parseParams(current.value),
        ...current,
      };

      if (existing) {
        if (Array.isArray(existing)) {
          existing.push(newValue);
        } else {
          previous.set(current.key, [existing, newValue]);
        }
      } else {
        previous.set(current.key, newValue);
      }

      return previous;
    }, new Map()),
  };
}

type ParsedHeader = {
  key: string;
  value: string;
  params: Record<string, string>;
};
export function getDkimSigners(
  mail: Email & {
    parsedHeaders: Map<string, ParsedHeader | ParsedHeader[]>;
  },
) {
  const dkimHeader = mail.parsedHeaders.get("dkim-signature");
  if (!dkimHeader) throw new DkimParsingError("No DKIM header found");
  if (Array.isArray(dkimHeader)) {
    return dkimHeader.map(parseHeader);
  }
  return [parseHeader(dkimHeader)];
}

export function parseParams(str: string) {
  return Object.fromEntries(
    str.split(";").map((s) =>
      s
        .trim()
        .split("=")
        .map((v) => v && v.trim()),
    ),
  );
}

function parseHeader(header: { params: Record<string, string> }) {
  if (!("params" in header)) {
    throw new DkimParsingError(`Invalid DKIM header ${header}`);
  }

  if (!header.params.d) {
    throw new DkimParsingError("DKIM header missing domain");
  }

  if (!header.params.s) {
    throw new DkimParsingError("DKIM header missing selector");
  }

  return {
    domain: header.params.d,
    selector: header.params.s,
  };
}
