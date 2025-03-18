import { getHeadersSize } from "./headers/headers";
import { CRLF_SIZE } from "./constants";
import { constraints } from "./constraints";
import { RequestSizeParams } from "./types";
import { getRequestLineSize } from "./requestLine/requestLine";
import { getBodySize } from "./body/body";

/**
 * Computes the total size of an HTTP request as it would be sent by TLSN.
 * This includes additional headers, the HTTP/1.1 version, and the full path in the request line.
 *
 * @param {RequestSizeParams} params - The parameters for the request size calculation.
 * @returns {number} The total size of the HTTP request in bytes.
 */
export function calculateRequestSize({
  url,
  method,
  headers,
  body,
}: RequestSizeParams) {
  const requestLineSize = getRequestLineSize(url, method, constraints);
  const bodySize = getBodySize(body);
  const headersSize = getHeadersSize(url, headers, body, constraints);
  return requestLineSize + headersSize + CRLF_SIZE + bodySize;
}
