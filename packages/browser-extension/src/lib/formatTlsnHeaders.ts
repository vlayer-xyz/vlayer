import browser from "webextension-polyfill";

// this is mechanism of extracting the headers and cookies from the request
// that works only for twitter
// we need to make it generic

export const formatTlsnHeaders = (
  headers: browser.WebRequest.HttpHeadersItemType[],
  cookies: browser.Cookies.Cookie[],
) => {
  const xCsrftoken =
    headers.find((header) => header.name === "x-csrf-token")?.value || "";
  const authToken =
    cookies.find((cookie) => cookie.name === "auth_token")?.value || "";
  const ct0 = cookies.find((cookie) => cookie.name === "ct0")?.value || "";
  const authorization =
    headers.find((header) => header.name === "authorization")?.value || "";

  if (!xCsrftoken || !authToken || !ct0 || !authorization) {
    return null;
  }
  return {
    headers: {
      "x-twitter-client-language": "en",
      "x-csrf-token": xCsrftoken,
      Host: "api.x.com",
      authorization: authorization,
      Cookie: `lang=en; auth_token=${authToken}; ct0=${ct0}`,
      "Accept-Encoding": "identity",
      Connection: "close",
    },
  };
};
