import { flow } from "fp-ts/lib/function";
import { RequestLineMode, CRLF } from "../constants";
import { getSize } from "../utils/getSize";

export const makeRequestLine = (
  url: string,
  method: string,
  constraints: {
    requestLineMode: RequestLineMode;
    protocol: string;
  },
) => {
  const fullUrl = new URL(url);
  return `${method} ${
    constraints.requestLineMode === RequestLineMode.FULL_PATH
      ? fullUrl.href
      : fullUrl.pathname
  } ${constraints.protocol}${CRLF}`;
};

export const getRequestLineSize = flow(makeRequestLine, getSize);
