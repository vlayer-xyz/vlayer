import { pipe } from "fp-ts/lib/function";
import {
  EncodedString,
  findAllQueryParams,
  NoGivenParamInUrlError,
} from "../utils";

const calculateRequestQueryParamsRanges = (
  urlQueryParams: string[],
  url: EncodedString,
  offset: number,
) => {
  const stepOverFirstAmpersand = 1;
  const stepOverEqualSign = 1;
  return urlQueryParams.map((param) => {
    const startInUrl =
      url.indexOf(`&${param}=`) !== -1
        ? url.indexOf(`&${param}=`)
        : url.indexOf(`?${param}=`);
    if (startInUrl === -1) {
      throw new NoGivenParamInUrlError(param);
    }
    const start =
      offset +
      startInUrl +
      stepOverFirstAmpersand +
      param.length +
      stepOverEqualSign;

    const secondAmpersandPosition = url.indexOf("&", start);
    const end =
      secondAmpersandPosition !== -1
        ? offset + secondAmpersandPosition
        : offset + url.length;

    return {
      start,
      end,
    };
  });
};

const getQueryParamsExcept = (url: string, url_query_except: string[]) => {
  return findAllQueryParams(url.toString()).filter(
    (param) => !url_query_except.includes(param),
  );
};

const calculateRequestQueryParamsRangesExcept = (
  url_query_except: string[],
  url: EncodedString,
  offset: number,
) => {
  return pipe(
    url.toString(),
    (urlStr) => getQueryParamsExcept(urlStr, url_query_except),
    (queryParams) =>
      calculateRequestQueryParamsRanges(queryParams, url, offset),
  );
};
export {
  calculateRequestQueryParamsRanges,
  getQueryParamsExcept,
  calculateRequestQueryParamsRangesExcept,
};
