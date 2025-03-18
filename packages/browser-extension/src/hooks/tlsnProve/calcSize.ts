export function calculateRequestSize({
  url,
  method,
  headers,
  body,
}: {
  url: string;
  method: string;
  headers: Record<string, string>;
  body?: unknown;
}) {
  const fullUrl = new URL(url);
  const requestLine = `${method} ${fullUrl.href} HTTP/1.1\r\n`;
  const requestLineSize = new TextEncoder().encode(requestLine).length;

  console.log("Request Line:", requestLine);

  // Ensure Host header is included

  // Body size in bytes
  const bodyContent = body ? JSON.stringify(body) : "";
  const bodySize = new TextEncoder().encode(bodyContent).length;

  console.log("Body:", bodyContent);

  const headersWithHost = {
    ...headers,
    Host: fullUrl.host,
    "Content-Length": bodySize.toString(),
    Connection: "close",
  };

  const headersString = Object.entries(headersWithHost)
    .map(([key, value]) => `${key}: ${value}\r\n`) // CRLF after each header
    .join("");

  console.log("Headers:", headersString);

  const headersSize = new TextEncoder().encode(headersString).length;

  const totalSize = requestLineSize + headersSize + 2 + bodySize; // +2 for CRLF after headers

  return {
    requestSize: totalSize,
    bodySize,
    headersSize,
  };
}
