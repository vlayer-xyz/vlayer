import { flow } from "fp-ts/lib/function";
import { CRLF } from "../constants";
import { Headers } from "../types";
import { getSize } from "../utils/getSize";

export const toHeadersString = (headers: Record<string, string>) => {
  return Object.entries(headers)
    .map(([key, value]) => `${key}: ${value}${CRLF}`)
    .join("");
};

export const makeHeaders = (
  url: string,
  headers: Record<string, string>,
  body: string | undefined,
  constraints: {
    defaultHeaders: Headers;
  },
) => {
  const generatedHeaders = Object.entries(constraints.defaultHeaders).reduce(
    (acc, [key, makeHeader]) => {
      acc[key] = makeHeader({ url, body });
      return acc;
    },
    {} as Record<string, string>,
  );

  //header should override default headers
  return {
    ...generatedHeaders,
    ...headers,
  };
};

export const getHeadersSize = flow(makeHeaders, toHeadersString, getSize);
