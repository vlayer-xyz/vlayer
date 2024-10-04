import browser from "webextension-polyfill";

export const formatTlsnHeaders = (
  headers: browser.WebRequest.HttpHeadersItemType[],
  cookies: browser.Cookies.Cookie[],
) => {
  return {
    headers: headers.reduce(
      (aggregatedHeaders, currentHeader) => {
        return {
          ...aggregatedHeaders,
          [currentHeader.name]: currentHeader.value,
        };
      },
      {
        //This is needed by TLSN as it doesn't work with compressed responses
        "Accept-Encoding": "identity",
        Cookie: cookies.reduce((aggregatedCookies, currentCookie) => {
          return `${aggregatedCookies}; ${currentCookie.name}=${currentCookie.value}`;
        }, ``),
      },
    ),
    secretHeaders: [],
  };
};
