const calculateRequestQueryParamsRanges = (
  urlQueryParams: string[],
  url: string,
  offset: number,
) => {
  return urlQueryParams.map((param) => {
    const stepOverFirstAmpersand = 1;
    const stepOverEqualSign = 1;

    const startInUrl =
      url.indexOf("&" + param + "=") !== -1
        ? url.indexOf("&" + param + "=")
        : url.indexOf("?" + param + "=");

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

export { calculateRequestQueryParamsRanges };
