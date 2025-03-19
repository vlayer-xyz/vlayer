import { getSize } from "./utils/getSize";

export const CRLF = "\r\n";
export const CRLF_SIZE = getSize(CRLF);

export enum HttpVersion {
  HTTP_1_1 = "HTTP/1.1",
}

export enum RequestLineMode {
  FULL_PATH,
  TARGET_ONLY,
}

export enum DefaultHeaders {
  Host = "Host",
  Connection = "Connection",
  ContentLength = "Content-Length",
}

export const DEFAULT_MAX_RECV_DATA = 16384;
