import dns from "node:dns";

export async function resolveDkimDns(domain: string, selector: string) {
  return new Promise<string>((resolve, reject) => {
    dns.resolveTxt(`${selector}._domainkey.${domain}`, (err, addresses) => {
      if (err) {
        reject(err);
        return;
      }

      resolve(addresses.flat()[0]);
    });
  });
}
