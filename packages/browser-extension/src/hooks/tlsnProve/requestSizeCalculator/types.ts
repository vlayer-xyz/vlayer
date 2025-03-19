import { RequestLineMode } from "./constants";

export type HeaderValueFactory = (args: {
  url: string;
  body?: string;
}) => string;

export type Headers = Record<string, HeaderValueFactory>;

export type RequestSizeParams = {
  url: string;
  method: string;
  headers: Record<string, string>;
  body?: string;
};

export type Constraints = {
  requestLineMode: RequestLineMode;
  protocol: string;
  defaultHeaders: Headers;
};
