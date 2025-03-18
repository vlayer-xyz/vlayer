import { RequestLineMode, DefaultHeaders, HttpVersion } from "./constants";
import { Constraints } from "./types";

export const constraints: Constraints = {
  requestLineMode: RequestLineMode.FULL_PATH,
  protocol: HttpVersion.HTTP_1_1,
  defaultHeaders: {
    [DefaultHeaders.Host]: ({ url }: { url: string }) => new URL(url).hostname,
    [DefaultHeaders.Connection]: () => "close",
    [DefaultHeaders.ContentLength]: ({ body }: { body?: string }) =>
      body ? new TextEncoder().encode(body).length.toString() : "0",
  },
};
